#[macro_use]
extern crate diesel;

#[macro_use]
extern crate derive_error;
pub use serde::{Serialize, Deserialize}; 

pub mod models;
pub mod schema;
mod errors;
use uuid::Uuid;
use bcrypt::{hash, verify};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use actix_web::{
    dev::ServiceRequest,
    get, post,
    web, http, dev, guard,
    App, HttpResponse, Error, client::Client,
    HttpServer, HttpRequest, Responder, ResponseError,
};
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use reqwest;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use actix_web::middleware::Logger;
extern crate env_logger;
use crate::schema::users_auth;

use diesel_migrations::run_pending_migrations;
use crate::models::{
    User,
    LoginUser,
    Claim,
};
use std::cell::Cell;

struct Counter {
    value: Cell<i32>,
}

impl Counter {
    pub fn new(value: i32) -> Counter {
        Counter{ value: Cell::new(value) }
    }
    pub fn inc(&self) -> i32 {
        self.value.set(self.value.get() + 1);
        self.value.get()
    }
}

#[get("/plus")]
pub async fn plus(c: web::Data<Counter> ) -> impl Responder {
    HttpResponse::Ok().body(c.inc().to_string())
}

use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;


pub async fn get(
    conn: web::Data<DbPool>,
) -> actix_web_dev::error::Result<HttpResponse> {
    let conn = conn.get()?;
    let r = User::get(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

#[derive (Deserialize)]
pub struct ID {
    id: i32,
}

pub async fn get_users(
    us: web::Json<Vec<ID>>,
    conn: web::Data<DbPool>,
) -> actix_web_dev::error::Result<HttpResponse> {
    let conn = conn.get()?;
    let us = us
        .into_inner()
        .into_iter()
        .map(|v|v.id)
        .collect::<Vec<i32>>();
    let r = User::get_users(&conn, us).await?;
    Ok(HttpResponse::Ok().json(r))
}
/*
pub async fn get_users(
    conn: web::Data<DbPool>
    ) -> actix_web_dev::error::Result<HttpResponse> {
    let conn = conn.get()?;
    let r = UserAuth::get(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}*/
/*
fn get_all_users
    (pool: web::Data<DbPool>
    ) -> Result<Vec<UserAuth>, Error> {
    let conn = pool.get().unwrap();
    let items = users_auth.load::<UserAuth>(&conn)?;
    Ok(items)
}
*/


pub async fn login(
    payload: web::Form<LoginUser>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let mail = payload.0.email;
    let pwd = payload.0.password;

    let err_message = Error::from(
        HttpResponse::Unauthorized().body("Wrong email/password")
    );
    
    let usr =  User::get_by_mail(&conn, mail).await.unwrap();

    if usr.password == pwd {

        let secret = String::from("APP_SECRET");
        let iat = Utc::now();
        let exp = iat + Duration::days(7);
        let claim = Claim {
                sub: usr.info,
                iat: iat.timestamp_nanos(),
                exp: exp.timestamp_nanos()
        };
        let token = encode(
            &Header::default(), 
            &claim, 
            &EncodingKey::from_secret(secret.as_ref())).unwrap();

        Ok(HttpResponse::Ok().body(token))
    } else {
         Err(err_message)
    }
}


pub async fn create_user(
    payload: web::Form<LoginUser>,
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();
    let mail = payload.0.email;
    let pwd = payload.0.password;

    let r = User::create_new_user(&conn, mail, pwd).await?;
    Ok(HttpResponse::Ok().json(r))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    match run_pending_migrations(&pool.get().unwrap()) {
        Ok(_) => print!("migration success\n"),
        Err(e)=> print!("migration error: {}\n",&e),
    };

    
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(Counter::new(0))
            .data(pool.clone())
            .service(plus)
            .route("/get", web::get().to(get))
            .route("/login", web::get().to(login))
            .route("/create_user", web::get().to(create_user))
            .route("/get_users", web::post().to(get_users))
  //          .route("/users", web::get().to(get_users))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


#[macro_use]
extern crate diesel;

#[macro_use]
extern crate derive_error;
pub use serde::{Serialize, Deserialize}; 

pub mod models;
pub mod schema;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use actix_web::{
    get, post,
    web, 
    App, HttpResponse, Error,
    HttpServer, HttpRequest, Responder, 
};
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
extern crate env_logger;

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
                sub: usr.id,
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


pub async fn update_user(
    req:  HttpRequest,
    info: web::Json<String>,
    conn: web::Data<DbPool>,
) -> actix_web_dev::error::Result<HttpResponse> {
    let conn = conn.get()?;
    let token = req
        .headers()
        .get("jwt")
        .unwrap()
        .to_str()
        .unwrap();
    
    let token = decode::<Claim>(&token, &DecodingKey::from_secret("APP_SECRET".as_ref()), &Validation::default())?;
    let r = User::update_user(&conn, token.claims.sub, info.into_inner()).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn make_friends(
    req:  HttpRequest,
    form: web::Json<String>,
    conn: web::Data<DbPool>,
) -> actix_web_dev::error::Result<HttpResponse> {
    let conn = conn.get()?;
    let token = req
        .headers()
        .get("jwt")
        .unwrap()
        .to_str()
        .unwrap();

    let token = decode::<Claim>(&token, &DecodingKey::from_secret("APP_SECRET".as_ref()), &Validation::default())?;
    let id_user = token.claims.sub;
    let r = User::make_friends(&conn, id_user, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn delete_friends(
    req:  HttpRequest,
    form: web::Json<String>,
    conn: web::Data<DbPool>,
) -> actix_web_dev::error::Result<HttpResponse> {
    let conn = conn.get()?;
    let token = req
        .headers()
        .get("jwt")
        .unwrap()
        .to_str()
        .unwrap();

    let token = decode::<Claim>(&token, &DecodingKey::from_secret("APP_SECRET".as_ref()), &Validation::default())?;
    let id_user = token.claims.sub;
    let r = User::delete_friends(&conn, id_user, form.into_inner()).await?;
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
            .route("/create_user", web::post().to(create_user))
            .route("/get_users", web::post().to(get_users))
            .route("/make_friends", web::put().to(make_friends))
            .route("/delete_friends", web::delete().to(delete_friends))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


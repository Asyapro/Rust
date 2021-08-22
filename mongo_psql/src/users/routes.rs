use actix_web::{
    web, http, dev, guard,
    App, HttpResponse, client::Client,
    HttpServer, HttpRequest, Responder,
};
use serde::Deserialize;
use serde::Serialize;
use futures_util::TryFutureExt;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use actix_web_dev::error::{
    Result,
    ApiError,
    ErrorType,
};
use actix_web_dev::auth::{
    Auth,
    AuthSecret,
};
use super::db::{
    AuthData,
    Customer,
};

pub async fn create(
    form: web::Json<AuthData>,
    conn: web::Data<DbPool>,
    _req: HttpRequest
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let form = form.into_inner();
    Auth::new(&form.mail.clone(), "plain", &vec![form.role.clone()], &conn).await?;
    Customer::new(&form, &conn).await?;
    Ok(HttpResponse::Ok().json(""))
}

pub async fn login(
    form: web::Json<AuthData>,
    secret: web::Data<AuthSecret>,
    conn: web::Data<DbPool>,
    req: HttpRequest
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let user = Customer::get(&form.into_inner(),&conn).await?;
    let login = user.mail;
    let a = Auth::get(&login, "plain", &conn).await?;
    let jwt = a.get_jwt(&secret).await?;
    Ok(HttpResponse::Ok().json(json!({"jwt":jwt})))
}

pub async fn get(
    auth: Auth,
    conn: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let r = Customer::from_mail(auth.login.clone(),&conn).await?;
    Ok(HttpResponse::Ok().json(json!({"user":r,"auth":auth})))
}


use futures_util::StreamExt as _;
pub async fn get_books(
    auth: Auth,
    cl: web::Data<std::sync::Mutex<mongodb::Client>>,
    conn: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let coll = cl.lock().unwrap().database("admin");
    let books: Collection<Book> = coll.collection_with_type("books");
    let mut res  = books.find(None, None).await.unwrap();
    let mut r: Vec<Book> = Vec::new();
    while let Some(doc) = res.next().await {
        let d: Book = doc.unwrap();
        r.push(d);
    }
    Ok(HttpResponse::Ok().json(r))
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Book {
    title: String,
    author: String,
    text: String,
}

use mongodb::Collection;
pub async fn add_book(
    auth: Auth,
    data: web::Json<Book>,
    cl: web::Data<std::sync::Mutex<mongodb::Client>>,
    req: HttpRequest
) -> Result<HttpResponse> {
    if auth.roles.contains(&"admin".to_string()) || 
        auth.roles.contains(&"teacher".to_string()) {
        let coll = cl.lock().unwrap().database("admin");
        let books: Collection<Book> = coll.collection_with_type("books");
        books.insert_one(data.clone(), None).await.unwrap();
        return Ok(HttpResponse::Ok().json(data.into_inner()));
    }
    Err(ApiError{
        code: 500,
        message: "no permissions".to_string(),
        error_type: ErrorType::Auth,
    })
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct BookTitle {
    title: String,
}

pub async fn rm_book(
    auth: Auth,
    data: web::Json<BookTitle>,
    cl: web::Data<std::sync::Mutex<mongodb::Client>>,
    req: HttpRequest
) -> Result<HttpResponse> {
    if auth.roles.contains(&"admin".to_string()) { 
        let coll = cl.lock().unwrap().database("admin");
        let books: Collection<Book> = coll.collection_with_type("books");
        books.delete_one(doc!{"title":data.title.clone()}, None)
            .await.unwrap();
        return Ok(HttpResponse::Ok().json(data.into_inner()));
    }
    Err(ApiError{
        code: 500,
        message: "no permissions".to_string(),
        error_type: ErrorType::Auth,
    })
}


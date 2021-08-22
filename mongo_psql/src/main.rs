use actix_web::{
    web, http, dev, guard,
    App, HttpResponse, client::Client,
    HttpServer,
};
use actix_web_dev::auth::*;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use actix_web::middleware::Logger;
use diesel_migrations::run_pending_migrations;
extern crate env_logger;
use books::users::routes::{
    create,
    login,
    get,
    rm_book,
    get_books,
    add_book,
};
use actix_web_dev::routes::{
    list,
    delete_user,
};

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
    let client = mongodb::Client::with_uri_str("mongodb://books_mongo:27017/")
        .await.unwrap();
    let db = client.database("admin");
    db.create_collection("books", None)
        .await;
    let client = web::Data::new(std::sync::Mutex::new(client));

    println!("created mongo collection");
    actix_web_dev::init_auth(&pool.get().unwrap());
    let secret = Auth::gen_secret();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("starting server...");
    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .data(pool.clone())
            .data(secret.clone())
            .wrap(Logger::default())
            .service(web::scope("/api")
                .service(web::scope("/user")
                    .route("/get", web::post().to(get))
                    .route("/login", web::post().to(login))
                    .route("/create", web::post().to(create))
                )
                .service(web::scope("/book")
                    .route("/get", web::post().to(get_books))
                    .route("/add", web::post().to(add_book))
                    .route("/rm", web::post().to(rm_book))
                )
                .service(web::scope("/auth")
                    .route("/list", web::post().to(list))
                    .route("/delete", web::post().to(delete_user))
                )
            )
    })
    .bind("0.0.0.0:8088")?
    .system_exit()
    .run()
    .await
}

use diesel::PgConnection;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate r2d2;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate env_logger;

pub(crate) mod schema;
pub mod error;
pub mod auth;
pub mod routes;

pub fn init_auth(connection: &PgConnection) {
    embed_migrations!("./migrations");
    embedded_migrations::run_with_output(connection, &mut std::io::stdout()).unwrap();
}

use actix_web_dev::error::{
    Result,
    ErrorType,
    ApiError,
};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::customer;
use ring::digest::{Context, Digest, SHA256};
use data_encoding::BASE64;

#[derive(Serialize,Deserialize,Clone,Queryable,AsChangeset)]
#[table_name = "customer"]
pub struct Customer {
    pub id: i64,
    pub mail: String,
    pub pass_hash: String,
    pub register_data: chrono::NaiveDateTime,
}

#[derive(Serialize,Deserialize,Clone)]
pub struct AuthData {
    pub mail: String,
    pub password: String,
    pub role: String,
}

fn make_hash(password: &str) -> String {
    let mut context = Context::new(&SHA256); 
    context.update(password.as_bytes());
    let pass_hash = context.finish();
    BASE64.encode(pass_hash.as_ref())
}

impl Customer {
    pub async fn new(
        creds: &AuthData, 
        conn: &PgConnection,
    ) -> Result<()> {
        diesel::insert_into(customer::table)
            .values(&(
                customer::pass_hash.eq(make_hash(&creds.password)),
                customer::mail.eq(&creds.mail),
            ))
            .execute(conn)?;
        Ok(())
    }

    pub async fn from_mail(
        mail: String,
        conn: &PgConnection,
    ) -> Result<Self> {
        let r = customer::table
            .filter(customer::mail.eq(mail))
            .get_results::<Self>(conn)?;
        if let Some(u) = r.get(0) {
            Ok(u.clone())
        } else {
            Err(ApiError{
                code: 404,
                message: "seller not found".to_string(),
                error_type: ErrorType::Auth,
            })
        }
    }

    pub async fn get(
        creds: &AuthData,
        conn: &PgConnection,
    ) -> Result<Self> {
        let pass_hash = make_hash(&creds.password);
        let r = customer::table
            .filter(customer::mail.eq(&creds.mail))
            .filter(customer::pass_hash.eq(pass_hash))
            .get_results::<Self>(conn)?;
        if let Some(u) = r.get(0) {
            Ok(u.clone())
        } else {
            Err(ApiError{
                code: 404,
                message: "seller not found".to_string(),
                error_type: ErrorType::Auth,
            })
        }
    }

    pub async fn update_pass(
        id: i64,
        new_pass: String,
        conn: &PgConnection,
    ) -> Result<()> {
        diesel::update(customer::table
            .filter(customer::id.eq(id)))
            .set(customer::pass_hash.eq(make_hash(&new_pass)))
            .execute(conn);
        Ok(())
    }
}

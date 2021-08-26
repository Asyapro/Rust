use actix_web_dev::error::{
    Result,
    ErrorType,
    ApiError,
};

use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sql_types::{
    Array,
    Int4,
    Varchar,
    Nullable,
    Serial,
};


use crate::schema::{
    users,
    users_auth,
};

#[derive(Serialize,Deserialize,Clone,Queryable,Debug)]
pub struct User {
    pub id: i32,
    pub info: String,
    pub friends: Option<Vec<i32>>,
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn get (conn: &PgConnection) -> actix_web_dev::error::Result<Vec<User>> {
        
        let r = users::table
            .get_results(conn)?;
        Ok(r)
    }
    
    pub async fn get_users (conn: &PgConnection, users: Vec<i32>) -> actix_web_dev::error::Result<Vec<User>> {

        let r = users::table
            .filter(users::id.eq_any(users))
            .get_results(conn)?;
        Ok(r)
    }


    pub async fn get_by_mail (conn: &PgConnection, mail: String) -> actix_web_dev::error::Result<User> {

        let r = users::table
            .filter(users::email.eq(mail))
            .get_result(conn)?;
        Ok(r)
    }

    pub async fn create_new_user (conn: &PgConnection, mail: String, pwd: String) -> actix_web_dev::error::Result<()> {

        let rows_inserted = diesel::insert_into(users::table)
                            .values(&(
                            users::info.eq(String::from("")),
                            users::email.eq(mail),
                            users::password.eq(pwd),
                            ))
                            .execute(conn)?;
        Ok(())
    }

}

/*
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserAuth {
    pub id: i32,
    pub nama: String,
    pub email: String,
    pub password: String,
}

impl UserAuth {

    pub async fn get (conn: &PgConnection) -> actix_web_dev::error::Result<Vec<UserAuth>> {

        let r = users_auth::table
            .get_results(conn)?;
        Ok(r)
    }

    pub async fn get_by_mail (conn: &PgConnection, mail: String) -> actix_web_dev::error::Result<UserAuth> {

        let r = users_auth::table
            .filter(users_auth::email.eq(mail))
            .get_result(conn)?;
        Ok(r)
    }

    pub async fn create_new_user (conn: &PgConnection, mail: String, pwd: String) -> actix_web_dev::error::Result<()> {
    
        let rows_inserted = diesel::insert_into(users_auth::table)
                             .values(&(
                             users_auth::email.eq(mail),
                             users_auth::password.eq(pwd),
                             ))
                             .execute(conn)?;
        Ok(())
    }

}
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String
}


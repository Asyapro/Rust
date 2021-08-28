use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::{
    users,
};

#[derive(Serialize,Deserialize,Clone,Queryable,Debug)]
pub struct User {
    pub id: i32,
    pub info: String,
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

        diesel::insert_into(users::table)
            .values(&(
            users::info.eq(String::from("")),
            users::email.eq(mail),
            users::password.eq(pwd),
            ))
            .execute(conn)?;

        Ok(())
    }
    
    
    pub async fn update_user (conn: &PgConnection, user: i32, info: String) -> actix_web_dev::error::Result<()> {
        
        let target = users::table.filter(users::id.eq(user));
        diesel::update(target)
            .set(users::info.eq(&info))
            .execute(conn)?;
        Ok(())
    }
    
    pub async fn make_friends (conn: &PgConnection, user1: i32, user2: String) -> actix_web_dev::error::Result<()> {

        let mut query = String::from("select * from make_friends (, );");
        query.insert_str(28, &user1.to_string());
        query.insert_str(31, &user2);

        diesel::sql_query(&query)
            .execute(conn)?;

        Ok(())
    }

    pub async fn delete_friends (conn: &PgConnection, user1: i32, user2: String) -> actix_web_dev::error::Result<()> {

        let mut query = String::from("select * from delete_friends (, );");
        query.insert_str(28, &user1.to_string());
        query.insert_str(31, &user2);
        println!("{}", query);

        diesel::sql_query(&query)
            .execute(conn)?;

        Ok(())
    }

}

#[derive(Serialize,Deserialize,Clone,Queryable)]
pub struct Claim {
    pub sub: i32,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String
}


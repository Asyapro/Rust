use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema::auth;
use crate::error::{
    Result,
    ErrorType,
    ApiError,
};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use rand::RngCore;

#[derive(Serialize,Deserialize,Queryable,Debug,Clone)]
pub struct Auth {
    pub id: i64,
    pub login: String,
    pub auth_type: String,
    pub roles: Vec<String>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct JwtClaim {
    pub user_data: Auth,
    pub exp: i64,
}

#[derive(Serialize,Deserialize, Clone,Default)]
pub struct AuthSecret(pub [u8;32]);

impl Auth {
    pub fn gen_secret() -> AuthSecret {
        let mut data = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut data);
        AuthSecret(data)
    }
    pub async fn new(
        login: &str,
        auth_type: &str,
        roles: &Vec<String>,
        conn: &PgConnection,
    ) -> Result<()>{
        diesel::insert_into(auth::table)
            .values(&(
                    auth::login.eq(login),
                    auth::auth_type.eq(auth_type),
                    auth::roles.eq(roles),
            ))
            .execute(conn)?;
        Ok(())
    }
    
    pub async fn list(
        conn: &PgConnection,
    ) -> Result<Vec<Auth>> {
        let r = auth::table
            .load(conn)?;
        Ok(r)
    }

    pub async fn delete(
        login: &str,
        conn: &PgConnection,
    ) -> Result<()> {
        diesel::delete(auth::table
            .filter(auth::login.eq(login)))
            .execute(conn)?;
        Ok(())
           
    }

    pub async fn get(
        login: &str,
        auth_type: &str,
        conn: &PgConnection,
    ) -> Result<Auth> {
        let r = auth::table
            .filter(auth::login.eq(login))
            .filter(auth::auth_type.eq(auth_type))
            .get_result(conn)?;
        Ok(r)
    }
    pub async fn get_jwt(
        &self,
        secret: &AuthSecret,
    ) -> Result<String> {
         let token = encode(
             &Header::default(), 
             &JwtClaim{
                 user_data: (*self).clone(),
                 exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
             }, 
             &EncodingKey::from_secret(&secret.0)
         )?;
         Ok(token)
    }
    pub fn validate_jwt(
        jwt: &str, 
        secret: &AuthSecret,
    ) -> Result<Auth> {
        let jwt = decode::<JwtClaim>(
            jwt,
            &DecodingKey::from_secret(&secret.0),
            &Validation::default()
        )?;
        if chrono::Utc::now().timestamp() >= jwt.claims.exp {
            return Err(ApiError {
                code: 401,
                message: "token is no longer valid".to_owned(),
                error_type: ErrorType::Auth,
            })
        }
        Ok(jwt.claims.user_data) 
    }
}


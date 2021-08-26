use crate::errors::ServiceError;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};
use std::error::Error;
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub fn validate_token(token: &str) -> Result<bool, ServiceError> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let jwks = fetch_jwks(&format!("{}{}", authority.as_str(), ".well-known/jwks.json"));
    let validations = vec![Validation::Issuer(authority), Validation::SubjectPresent];
    let kid = match token_kid(&token) {
        Ok(res) => res,
        Err(_) => return Err(ServiceError::JWKSFetchError),
    };
    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    let res = validate(token, jwk, validations);
    Ok(res.is_ok())
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let mut res = reqwest::get(uri).await?;
    let val = res.json::<JWKS>().await?;
    return Ok(val);
}

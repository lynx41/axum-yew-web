use axum::http::StatusCode;
use dotenvy_macro::dotenv;
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey, decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};

use super::app_error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_token() -> Result<String, AppError> {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    // let iat = now as usize;
    let exp = (now + chrono::Duration::seconds(10)).timestamp() as usize;
    
    // timestamp now
    // let mut now = chrono::Utc::now();
    
    // // iat i64 to usize
    // let iat = now.timestamp() as usize;
    
    // // // expires in that time
    // // let expires_in = chrono::Duration::seconds(30);
    
    // let exp = (now + chrono::Duration::seconds(10)).timestamp() as usize;
    // // timestamp + expire time
    // // now += expires_in;
    
    // write result as exp time (i64 to usize)
    // let exp = now.timestamp() as usize;

    let claim = Claims { exp, iat };

    let secret = EncodingKey::from_secret({
        dotenv!("JWT_SECRET").as_bytes()
    });

    // dbg!(dotenv!("JWT_SECRET"));
    // dbg!(dotenv!("JWT_SECRET").as_bytes());

    let token = encode(
        &Header::default(),
        &claim,
        &secret)
        .map_err(|e| {
            dbg!(e);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error"
            )
        })?;

    Ok(token)
}

pub fn is_valid(
    token: &str
) -> Result<bool, AppError> {
    let key = DecodingKey::from_secret({
        dotenv!("JWT_SECRET").as_bytes()
    });
    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(token, &key, &validation)
        .map_err(|error| {
            match error.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    // If signature is expired
                    AppError::new(StatusCode::UNAUTHORIZED, "Your session has expired, please login again")
                }
                _ => AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            }
        })?;

    Ok(true)
} 
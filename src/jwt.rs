use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, TokenData};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use dotenvy::dotenv;
use std::env;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u64, 
    pub exp: usize,
}

pub fn create_jwt(user_id: u64, ttl_seconds: i64) -> String {

    dotenv().ok();
    let secret_key = env::var("JWT_SECRET").expect("JWT SECRET must be set");
 
    let exp = Utc::now()
        .checked_add_signed(Duration::seconds(ttl_seconds))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims { sub: user_id, exp };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_bytes()))
        .expect("JWT encode failed")
}

pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    
     dotenv().ok();
     let secret_key = env::var("JWT_SECRET").expect("JWT SECRET must be set");
     
    decode::<Claims>(token, &DecodingKey::from_secret(secret_key.as_bytes()), &Validation::default())
}

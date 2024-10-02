use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

pub fn generate_token(id: i32) -> String {
    let expr = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 60 * 60;
    let claims = Claims {
        sub: id,
        exp: expr as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(std::env::var("SECRET_KEY").expect("SECRET_KEY must be set").as_ref())).unwrap()
}

pub fn authenticate(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret(std::env::var("SECRET_KEY").expect("SECRET_KEY must be set").as_ref()), &Validation::new(Algorithm::HS256))
        .map(|data| data.claims)
}

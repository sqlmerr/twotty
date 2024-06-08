use crate::schemas::auth::Claims;

use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{
    decode, encode, errors::Result, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use once_cell::sync::Lazy;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub fn create_token(claims: &Claims) -> Result<String> {
    encode(&Header::default(), claims, &KEYS.encoding)
}

pub fn decode_token(token: &str) -> Result<TokenData<Claims>> {
    decode::<Claims>(token, &KEYS.decoding, &Validation::default())
}

pub fn hash_password(password: String) -> String {
    hash(password, DEFAULT_COST).unwrap().to_string()
}

pub fn verify_password(password: String, hashed_password: String) -> bool {
    verify(password, hashed_password.as_str()).unwrap()
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

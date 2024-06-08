use crate::schemas::auth::Claims;
use crate::Config;
use jsonwebtoken::{
    decode, encode, errors::Result, DecodingKey, EncodingKey, Header, TokenData, Validation,
};

pub fn create_token(claims: &Claims) -> Result<String> {
    let config = Config::from_env();

    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
}

pub fn decode_token(token: &str) -> Result<TokenData<Claims>> {
    let config = Config::from_env();

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
}

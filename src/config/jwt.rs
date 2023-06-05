use crate::config::config::get_env;
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub id: i64,
    pub exp: usize,
}

pub fn create_token(id: i64) -> String {
    let one_year_from_now = SystemTime::now() + std::time::Duration::from_secs(60 * 60 * 24 * 365);
    let exp = one_year_from_now
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let claim: Claim = Claim { id, exp };
    let token: String = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(get_env("SECRET_KEY").to_owned().as_ref()),
    )
    .unwrap();
    token
}

pub fn validate_token(token: String) -> Result<TokenData<Claim>, Error> {
    let data = decode::<Claim>(
        &token,
        &DecodingKey::from_secret(get_env("SECRET_KEY").to_owned().as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(data)
}

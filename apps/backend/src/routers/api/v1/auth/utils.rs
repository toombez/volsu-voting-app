const SECRET: &str = "a6cc0a06a7add9158c38f865de8fc1c91f8fbafbee6eed120ac2d16aa0d5d0e6ce3f0596b9ce6c876c532c9721e7683c6b808cf4a612ed4c17d68802b175b6e8e8ab9c283d9fd2b896407691dbe497132df3dc612dcab08cf6bf0af816a3e0b0a5183bde187996b64e2bbbc796c019497922be4556de1622b4d707c4b35460b51db03dfcfe5b4f14ba4bbd2009fb09e89252a761ab492d5118da14017f9f4e657685220d9e65bab31d9f25435eac3e1255f1096a019724094f4dda5f56d19ffb33021cea394008715ca778ce1e1ba3f72580642bba15043b0c1c04c356428c46be1a9765611334571b9c4ecce40176ddb0209f59fd67bceac31429849a183423";

use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, TokenData, Validation, errors::Error as JWTError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub username: String,
    pub id: String,
}

pub fn encode_jwt(username: String, id: String) -> Result<String, JWTError> {
    let secret: String = SECRET.to_string();
    let now = Utc::now();

    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims { iat, exp, username, id };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}


pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, JWTError> {
    let secret = SECRET.to_string();

    decode(
        &jwt_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
}

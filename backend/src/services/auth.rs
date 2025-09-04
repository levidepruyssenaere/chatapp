use std::time::{Duration, SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, EncodingKey, Header, errors::Result as JWTResult, DecodingKey, Validation, decode};
use mongodb::Database;
use rocket::State;
use crate::db::users::get_user_by_email;
use crate::models::claims::Claims;

const SECRET: &[u8] = b"09120312";
pub fn generate_jwt(email: &String) -> String {
    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(60 * 60))
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    
    let claims = Claims {
        sub: email.to_owned(),
        exp: expiration,
    };
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)).unwrap()
}

pub fn verify_jwt(token: &str) -> JWTResult<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub async fn authenticate(db: &State<Database>, email: &String, password: &String) -> Result<String, String> {
    match get_user_by_email(db, email).await {
        Ok(user) => {
            if user.password == *password {
                Ok(generate_jwt(email))
            } else {
                Err("Invalid Password".to_string())
            }
        },
        Err(e) => Err(e),
    }
}
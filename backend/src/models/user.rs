use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, )]
pub struct User {
    pub email: String,
    pub username: String,
    pub password: String,
}
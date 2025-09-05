use mongodb::bson::doc;
use mongodb::Database;
use mongodb::results::InsertOneResult;
use rocket::{State};
use crate::models::user::User;

pub async fn get_user_by_email(db: &State<Database>, email: &String) -> Result<User, String> {
    let collection = db.collection::<User>("users");
    match collection.find_one(doc! {"email": email}).await {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(format!("No user found with email {}", email)),
        Err(e) => Err(format!("Database error: {}", e))
    }
}

pub async fn get_user_by_username(db: &State<Database>, username: &String) -> Result<User, String> {
    let collection = db.collection::<User>("users");
    match collection.find_one(doc! {"username": username}).await {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(format!("No user found with username {}", username)),
        Err(e) => Err(format!("Database error: {}", e))
    }
}

pub async fn create_user(db: &State<Database>, email: &String, username: &String, password: &String) -> Result<InsertOneResult, String> {
    let collection = db.collection::<User>("users");
    let user = User {
        email: email.clone(),
        username: username.clone(),
        password: password.clone(),
    };
    let created_user = match collection.insert_one(user).await {
        Ok(created_user) => Ok(created_user),
        Err(e) => Err(format!("Failed to create user {}", e.to_string()))
    };
    created_user
}
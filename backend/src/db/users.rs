use mongodb::bson::doc;
use mongodb::Database;
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
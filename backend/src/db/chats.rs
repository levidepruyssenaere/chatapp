use chrono::Local;
use mongodb::bson::doc;
use mongodb::Database;
use rocket::futures::TryStreamExt;
use rocket::serde::json::Json;
use rocket::State;
use crate::models::message::Message;

pub async fn get_public_chats_db(db: &State<Database>) -> Json<Vec<Message>>  {
    let collection = db.collection::<Message>("public_messages");
    let dt = Local::now();
    let message: Message = Message {
        id: None,
        message: "Sup dawg".to_string(),
        author: "Jan Deploige".to_string(),
        timestamp: dt.to_string()
    };
    collection.insert_one(message)
        .await
        .expect("Failed to insert into Collection");
    let cursor = collection.find(doc! {})
        .await
        .expect("Failed to get from collection");
    let messages: Vec<Message> = cursor
        .try_collect()
        .await
        .expect("Failed to convert from Collection");

    Json(messages)
}
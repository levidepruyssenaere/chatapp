use chrono::Local;
use crate::models::message::Message;

pub fn get_public_mock_messages() -> Vec<Message> {
    let dt = Local::now();
    let mock_messages: Vec<Message> = vec![
        Message {
            id: None,
            message: String::from("Hey Levi alles goed?"),
            author: String::from("Jan Deploige"),
            timestamp: dt.timestamp().to_string(),
        },
        Message {
            id: None,
            message: String::from("Ja hoor met mij alles prima!"),
            author: String::from("Levi De Pruyssenaere de la Woestijne"),
            timestamp: dt.timestamp().to_string(),
        }
    ];
    mock_messages
}
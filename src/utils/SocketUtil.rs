use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use tokio::net::TcpStream;
use futures_util::{StreamExt, SinkExt};
use sea_orm::DatabaseConnection;
use serde_json::Value;
use crate::utils::MessagingUtil::save_message;

pub async fn handle_connection(
    db: Arc<DatabaseConnection>,
    stream: TcpStream,
    tx: Arc<Mutex<broadcast::Sender<String>>>,
) {
    let ws_stream = accept_async(stream).await.expect("WebSocket handshake failed");
    let (mut write, mut read) = ws_stream.split();

    let mut rx = tx.lock().await.subscribe(); // Await the mutex lock

    // Spawn a task to forward broadcast messages to this client
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Err(e) = write.send(Message::Text(msg)).await {
                eprintln!("Error sending message: {:?}", e);
                break;
            }
        }
    });

    // Read messages from the client and broadcast them
    while let Some(Ok(Message::Text(text))) = read.next().await {
        if let Ok(parsed_message) = serde_json::from_str::<Value>(&text) {
            println!("Received message: {:?}", parsed_message);

            let mut tx_guard = tx.lock().await; // Await the mutex lock

            if let Err(e) = tx_guard.send(text.clone()) {
                eprintln!("Error broadcasting message: {:?}", e);
            } else {
                println!("Broadcasted message: {}", text);

                // Save the message to the database
                if let (Some(user_id), Some(chat_id), Some(content)) = (
                    parsed_message["user_id"].as_i64(),
                    parsed_message["chat_id"].as_i64(),
                    parsed_message["message"].as_str(),
                ) {
                    if let Err(e) = save_message(
                        db.clone(),
                        user_id as i32,
                        chat_id as i32,
                        content.to_string(),
                    )
                        .await
                    {
                        eprintln!("Error saving message to database: {:?}", e);
                    } else {
                        println!("Message saved to database.");
                    }
                }
            }
        }
    }
}

mod db;
mod entity;
mod utils;
mod controllers;

use sea_orm::{ActiveModelTrait, DatabaseConnection, DbConn, EntityTrait};
use crate::db::db::establish_connection;
use crate::utils::SocketUtil::handle_connection;
use tokio::net::TcpListener;
use futures_util::{StreamExt, SinkExt};
use actix_web::{web, App, HttpResponse, HttpServer};
use bcrypt::hash;
use sea_orm::ActiveValue::Set;
use serde::{Serialize, Deserialize};
use warp::Filter;
use crate::controllers::auth_controller::{login, LoginRequest};
use crate::entity::user;
use crate::utils::MessagingUtil::get_all_chat_messages;
use crate::utils::UserUtil::update_password;
use tokio::sync::{broadcast, Mutex};
use std::sync::Arc;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = Arc::new(establish_connection().await?);
    println!("Database connection established!");

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on ws://127.0.0.1:8080");

    // Use tokio::sync::Mutex for the broadcast channel
    let (tx, _) = broadcast::channel::<String>(100); // Channel size is 100
    let tx = Arc::new(Mutex::new(tx));

    // Handle incoming WebSocket connections
    let pool_clone = Arc::clone(&pool);
    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            let tx = Arc::clone(&tx);
            let pool = Arc::clone(&pool_clone); // Clone the Arc to share the connection
            tokio::spawn(handle_connection(pool, stream, tx));
        }
    });
    // Define the routes for HTTP server
    let login_route = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and_then(login);

    let chat_route = warp::path!("get-chat-messages" / i32)
        .and(warp::get())
        .and(warp::any().map(move || pool.clone()))
        .and_then(|chat_id, db: Arc<DatabaseConnection>| async move {
            println!("Fetching messages for chat_id: {}", chat_id);
            match get_all_chat_messages(&db, chat_id).await {
                Ok(messages) => {
                    println!("Fetched messages: {:?}", messages);
                    Ok(warp::reply::json(&messages)) as Result<_, warp::Rejection>
                }
                Err(err) => {
                    eprintln!("Error fetching messages: {:?}", err);
                    Err(warp::reject::not_found())
                }
            }
        });

    let routes = login_route.or(chat_route);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["Authorization", "Content-Type"]);

    warp::serve(routes.with(cors))
        .run(([127, 0, 0, 1], 3000))
        .await;

    Ok(())
}
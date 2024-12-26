use std::sync::Arc;
use sea_orm::{EntityTrait, Set, ActiveModelTrait, DbConn, DatabaseConnection, QueryOrder, FromQueryResult};
use sea_orm::prelude::DateTime;
use crate::entity::{chat_user, message, chat};
use sea_orm::QueryFilter;
use serde::{Deserialize, Serialize};
use sea_orm::ColumnTrait;

#[derive(FromQueryResult, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub chat_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: DateTime,
}

pub async fn save_message(db: Arc<DatabaseConnection>, user_id: i32, chat_id: i32, content: String) -> Result<(), sea_orm::DbErr> {
    let new_message = message::ActiveModel {
        user_id: Set(user_id),
        chat_id: Set(chat_id),
        content: Set(content),
        ..Default::default()
    };

    message::Entity::insert(new_message)
        .exec(&*db)
        .await?;

    Ok(())
}


pub async fn create_chat(db: &DbConn, user_ids: Vec<i32>, chat_name: String) -> Result<(), sea_orm::DbErr> {
    let new_chat = chat::ActiveModel {
        name: Set(chat_name),
        ..Default::default()
    };

    let inserted_chat = chat::Entity::insert(new_chat).exec(db).await?;

    for user_id in user_ids {
        let new_chat_user = chat_user::ActiveModel {
            chat_id: Set(inserted_chat.last_insert_id),
            user_id: Set(user_id),
            ..Default::default()
        };

        chat_user::Entity::insert(new_chat_user).exec(db).await?;
    }

    Ok(())
}

pub async fn get_all_chat_messages(
    db_conn: &DatabaseConnection,
    chat_id: i32,
) -> Result<Vec<Message>, Box<dyn std::error::Error>> {

    let messages = message::Entity::find()
        .filter(message::Column::ChatId.eq(chat_id)) // Use `ColumnTrait` for `eq`
        .into_model::<Message>() // Map results into the Message model
        .all(db_conn) // Execute the query
        .await?;
    Ok(messages)
}
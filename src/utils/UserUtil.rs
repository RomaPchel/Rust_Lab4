use std::error::Error;
use actix_web::{web, HttpResponse, Responder};
use sea_orm::{EntityTrait, FromQueryResult, QueryFilter, ColumnTrait, DatabaseConnection, ActiveModelTrait};
use sea_orm::ActiveValue::Set;
// Import ColumnTrait
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use crate::db::db::establish_connection;
use crate::entity::user;
use crate::entity::user::ActiveModel;

#[derive(FromQueryResult, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime,
}

pub async fn get_user_by_user_name(username: String) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let pool: sea_orm::DatabaseConnection = establish_connection().await?;

    // Correct use of `filter` after importing `ColumnTrait`
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(username.clone()))  // `eq` method now available
        .into_model::<User>()
        .one(&pool)  // Use `&pool` instead of cloning
        .await?;

    Ok(user)
}

#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
    username: String,
    new_password: String,
}


pub async fn update_password(
    db: &DatabaseConnection, // Database connection
    username: &str,          // The username of the user whose password needs to be updated
    new_password: &str,      // The new password to set
) -> Result<(), String> {
    // Step 1: Fetch the user from the database by username
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await;

    match user {
        Ok(Some(mut user)) => {
            // Step 2: Update the user's password
            let mut active_model: ActiveModel = user.into(); // Convert the user to an ActiveModel
            active_model.password = Set(new_password.to_string()); // Set the new password

            // Step 3: Save the updated user back to the database
            let result = active_model.update(db).await;

            match result {
                Ok(_) => Ok(()), // Password update successful
                Err(_) => Err("Error updating password".to_string()), // Error updating password
            }
        }
        Ok(None) => Err("User not found".to_string()), // User not found
        Err(_) => Err("Error querying database".to_string()), // Database query error
    }
}

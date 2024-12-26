
use sea_orm::{EntityTrait, FromQueryResult, QueryFilter, ColumnTrait, DatabaseConnection, ActiveModelTrait};
use sea_orm::ActiveValue::Set;
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

    let user = user::Entity::find()
        .filter(user::Column::Username.eq(username.clone()))
        .into_model::<User>()
        .one(&pool)
        .await?;

    Ok(user)
}


#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
    username: String,
    new_password: String,
}



pub async fn update_password(
    db: &DatabaseConnection,
    username: &str,
    new_password: String,
) -> Result<(), String> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await;

    match user {
        Ok(Some(mut user)) => {
            let mut active_model: ActiveModel = user.into();
            active_model.password = Set(new_password.to_string());

            let result = active_model.update(db).await;

            match result {
                Ok(_) => Ok(()),
                Err(_) => Err("Error updating password".to_string()),
            }
        }
        Ok(None) => Err("User not found".to_string()),
        Err(_) => Err("Error querying database".to_string()),
    }
}

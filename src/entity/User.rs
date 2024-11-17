use sea_orm::entity::prelude::*;
use crate::entity::{chat_user, message, user};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Message,
    ChatUser,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Message => Entity::belongs_to(message::Entity)
                .from(Column::Id)
                .to(message::Column::Id)
                .into(),
            Self::ChatUser => Entity::belongs_to(chat_user::Entity)
                .from(Column::Id)
                .to(chat_user::Column::UserId)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

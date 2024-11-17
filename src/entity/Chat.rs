use sea_orm::entity::prelude::*;
use crate::entity::{chat_user, message};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "chats")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    ChatUser,
    Message,
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
                .to(chat_user::Column::ChatId)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

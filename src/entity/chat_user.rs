use sea_orm::entity::prelude::*;
use crate::entity::user;
use crate::entity::chat;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "chat_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub chat_id: i32,
    #[sea_orm(primary_key)]
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Chat,
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Chat => Entity::belongs_to(chat::Entity)
                .from(Column::ChatId)
                .to(chat::Column::Id)
                .into(),
            Self::User => Entity::belongs_to(user::Entity)
                .from(Column::UserId)
                .to(user::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

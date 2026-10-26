use std::str::FromStr;

use sea_orm::entity::prelude::*;
use teloxide::types::User;


//user model
#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, Default, DeriveEntityModel)]
#[sea_orm(table_name ="users")]
pub struct Model{
    #[sea_orm(primary_key)]
    id: u64,
    tg_user_id: i64,
    tg_chat_id: i64,
    name: String,

    #[sea_orm(has_many)]
    recipes: HasMany<super::recipe::Entity>,

    #[sea_orm(has_many)]
    folders: HasMany<super::folder::Entity>
}

impl Model {
   pub fn new(tg_user_id: i64, tg_chat_id: i64, name: &str) -> Self{    
        Model {tg_user_id, tg_chat_id, name: name.to_string()..Default::default() }
   }
}

impl ActiveModelBehavior for ActiveModel{

}

// #[sea_orm(
//     belongs_to = "super::team::Entity",
//     from = "Column::TeamId",
//     to = "super::team::Column::Id",
//     on_update = "NoAction",
//     on_delete = "NoAction"
// )]


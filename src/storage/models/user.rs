use sea_orm::entity::prelude::*;


//user model
#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, Default, DeriveEntityModel)]
#[sea_orm(table_name ="users")]
pub struct Model{
    #[sea_orm(primary_key)]
    id: u64,
    name: String,

    #[sea_orm(has_many)]
    recipes: HasMany<super::recipe::Entity>
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


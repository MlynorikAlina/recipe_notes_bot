use sea_orm::entity::prelude::*;


//user model
#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, Default, DeriveEntityModel)]
#[sea_orm(table_name ="recipes")]
pub struct Model{
    #[sea_orm(primary_key)]
    id: u64,
    user_id: u64,
    folder_id: u64,
    ingridients: String,
    notes: String,


    #[sea_orm(belongs_to, from ="user_id", to = "id", on_update="Cascade", on_delete="Cascade")]
    user: BelongsTo<super::user::Entity>,    
    #[sea_orm(belongs_to, from ="folder_id", to = "id", on_update="Cascade", on_delete="Cascade")]
    folder: BelongsTo<super::folder::Entity>    
}

impl ActiveModelBehavior for ActiveModel{

}


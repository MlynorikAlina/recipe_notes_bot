use sea_orm::{ActiveValue::Set, entity::prelude::*};


//user model
#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, Default, DeriveEntityModel)]
#[sea_orm(table_name ="recipes")]
pub struct Model{
    #[sea_orm(primary_key)]
    id: u64,
    user_id: i64,
    folder_id: u64,
    ingridients: String,
    notes: String,


    #[sea_orm(belongs_to, from ="user_id", to = "tg_user_id", on_update="Cascade", on_delete="Cascade")]
    user: BelongsTo<super::user::Entity>,    
    #[sea_orm(belongs_to, from ="folder_id", to = "id", on_update="Cascade", on_delete="Cascade")]
    folder: BelongsTo<super::folder::Entity>    
}

impl ActiveModel {
    pub fn new(
        tg_user_id: i64,
        save_folder_id: u64,
        ingridients: &str,
        notes: &str
    )->Self{
        ActiveModel { 
            user_id: Set(tg_user_id),
            folder_id: Set(save_folder_id),
            ingridients: Set(ingridients.to_owned()),
            notes: Set(notes.to_owned()),
            ..Default::default()
        }
    }
    
}

impl ActiveModelBehavior for ActiveModel{

}


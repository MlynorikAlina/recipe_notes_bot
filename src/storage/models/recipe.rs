use sea_orm::{ActiveValue::Set, entity::prelude::*};


//user model
#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, Default, DeriveEntityModel)]
#[sea_orm(table_name ="recipes")]
pub struct Model{
    #[sea_orm(primary_key)]
    id: u64,
    user_id: i64,
    folder_id: Option<u64>,
    title: String,
    ingridients: String,
    notes: String,


    #[sea_orm(belongs_to, from ="user_id", to = "tg_user_id", on_update="Cascade", on_delete="Cascade")]
    user: BelongsTo<super::user::Entity>,    
    #[sea_orm(belongs_to, from ="folder_id", to = "id", on_update="Cascade", on_delete="Cascade")]
    folder: BelongsTo<Option<super::folder::Entity>>    
}

impl ActiveModel {
    pub fn new(
        tg_user_id: i64,
        save_folder_id: Option<u64>,
        title: &str,
        ingridients: &str,
        notes: &str
    )->Self{
        ActiveModel { 
            user_id: Set(tg_user_id),
            folder_id: Set(save_folder_id),
            title: Set(title.to_owned()),
            ingridients: Set(ingridients.to_owned()),
            notes: Set(notes.to_owned()),
            ..Default::default()
        }
    }

    pub fn rename(&mut self, new_title: &str)->&Self{
        self.title = Set(new_title.to_owned());
        self
    }

    pub fn move_to_dir(&mut self, parent: Option<u64>) -> &Self{
        self.folder_id = Set(parent);
        self
    }

    pub fn change_ingridients(&mut self, ingridients: &str)->&Self{
        self.ingridients = Set(ingridients.to_owned());
        self
    }

    pub fn change_notes(&mut self, notes: &str)->&Self{
        self.title = Set(notes.to_owned());
        self
    }
    
}

impl ActiveModelBehavior for ActiveModel{

}


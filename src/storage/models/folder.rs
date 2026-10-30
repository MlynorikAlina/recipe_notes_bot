use sea_orm::{ActiveValue::Set, entity::prelude::*};


#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name="folders")]
pub struct Model{
    #[sea_orm(primary_key)]
    id:u64,
    parent_id: Option<u64>,
    user_id: i64,
    name: String,

    #[sea_orm(self_ref, from = "parent_id", to = "id", relation_enum="Parent", relation_reverse="Children")]
    parent: BelongsTo<Option<Entity>>,

    #[sea_orm(self_ref, relation_enum="Children", relation_reverse="Parent")]
    children: HasMany<Entity>,

    #[sea_orm(has_many)]
    recipes: HasMany<super::recipe::Entity>,

    #[sea_orm(belongs_to, from="user_id", to="tg_user_id", on_delete="Cascade")]
    user: BelongsTo<super::user::Entity>
}

impl ActiveModel {
    pub fn new(
        tg_user_id: i64,
        parent_folder_id: Option<u64>,
        folder_name: &str
    )->Self{
        ActiveModel { 
            user_id: Set(tg_user_id),
            parent_id: Set(parent_folder_id),
            name: Set(folder_name.to_owned()),
            ..Default::default()
        }
    }

    pub fn rename(&mut self, new_name: &str) -> &Self{
        self.name = Set(new_name.to_owned());
        self
    }

    pub fn move_to_dir(&mut self, parent: Option<u64>) -> &Self{
        self.parent_id = Set(parent);
        self
    }
    
}


impl ActiveModelBehavior for ActiveModel {
    
}


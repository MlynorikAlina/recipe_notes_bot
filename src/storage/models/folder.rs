use sea_orm::entity::prelude::*;


#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name="folders")]
pub struct Model{
    #[sea_orm(primary_key)]
    id:u64,
    parent_id: Option<u64>,
    user_id: u64,
    name: String,

    #[sea_orm(self_ref, from = "parent_id", to = "id", relation_enum="Parent", relation_reverse="Children")]
    parent: BelongsTo<Option<Entity>>,

    #[sea_orm(self_ref, relation_enum="Children", relation_reverse="Parent")]
    children: HasMany<Entity>,

    #[sea_orm(has_many)]
    recipes: HasMany<super::recipe::Entity>,

    #[sea_orm(belongs_to, from="user_id", to="id", on_delete="Cascade")]
    user: BelongsTo<super::user::Entity>
}

impl ActiveModelBehavior for ActiveModel {
    
}


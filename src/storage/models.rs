mod user;
mod recipe;
mod folder;

//https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-format/

pub use user::{ActiveModel as UserActiveModel, Model as UserModel, Entity as User};
pub use recipe::{ActiveModel as RecipeActiveModel, Model as RecipeModel, Entity as Recipe};
pub use folder::{ActiveModel as FolderActiveModel, Model as FolderModel, Entity as Folder};

pub trait ActiveModelData {
    
}

impl ActiveModelData for UserActiveModel {
    
}

impl ActiveModelData for FolderActiveModel {
    
}

impl ActiveModelData for RecipeActiveModel {
    
}

// pub struct User{

// }

// pub struct Folder{

// }

// pub struct Tag{

// }

// pub struct Recipe{

// }
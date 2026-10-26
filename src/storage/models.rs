mod user;
mod recipe;
mod folder;

pub use user::{ActiveModel as UserActiveModel, Model as UserModel, Entity as User};
pub use recipe::{ActiveModel as RecipeActiveModel, Model as RecipeModel, Entity as Recipe};
pub use folder::{ActiveModel as FolderActiveModel, Model as FolderModel, Entity as Folder};

// pub struct User{

// }

// pub struct Folder{

// }

// pub struct Tag{

// }

// pub struct Recipe{

// }
mod user;
mod recipe;
mod folder;

pub use user::{ActiveModel as UserActive, Model as User};
pub use recipe::{ActiveModel as RecipeActive, Model as Recipe};
pub use folder::{ActiveModel as FolderActive, Model as Folder};

// pub struct User{

// }

// pub struct Folder{

// }

// pub struct Tag{

// }

// pub struct Recipe{

// }
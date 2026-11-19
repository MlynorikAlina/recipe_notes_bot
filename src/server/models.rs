use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FSObject{
    pub id:String,
    pub name: String,
    pub obj_type: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FSFolder{
    pub user_id: String,
    pub parent_id: Option<String>,
    pub name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FSFile{
    pub user_id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub ingridients: String,
    pub description: String
}

pub enum FSObjectType{
    Dir,
    File
}

impl FSObject{
    pub fn new(id:u64, name: &str, obj_type: FSObjectType)-> Self{
        let obj_type= match obj_type {
            FSObjectType::Dir => "dir".to_owned(),
            FSObjectType::File => "file".to_owned(),            
        }; 
        FSObject{
            id: id.to_string(),
            obj_type,
            name: name.to_owned(),
        }
    }
}
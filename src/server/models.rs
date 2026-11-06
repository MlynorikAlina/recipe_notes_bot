use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FSObject{
    pub id:String,
    pub parent_id: Option<u64>,
    pub name: String,
    pub obj_type: String
}

pub enum FSObjectType{
    Dir,
    File
}

impl FSObject{
    pub fn new(id:u64, parent_id: Option<u64>, name: &str, obj_type: FSObjectType)-> Self{
        let obj_type= match obj_type {
            FSObjectType::Dir => "dir".to_owned(),
            FSObjectType::File => "file".to_owned(),            
        }; 
        FSObject{
            id: id.to_string(),
            parent_id,
            obj_type,
            name: name.to_owned(),
        }
    }
}
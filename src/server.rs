use rocket::{get, post, serde::json::Json};

mod models;

#[get("/ping")]
pub fn ping()->Json<models::FSObject>{
    Json::from(models::FSObject::new(123, "xiao", models::FSObjectType::Dir))
}

#[get("/get/fs?<user_id>&<folder_id>")]
pub fn get_fs(user_id: String, folder_id: Option<String>)->Json<Vec<models::FSObject>>{
    println!("State {} dir :{:?}", &user_id, &folder_id);
    if folder_id != None {
        Json::from(vec![
        models::FSObject::new(2, "123xiao", models::FSObjectType::Dir),
        models::FSObject::new(3, "3bao", models::FSObjectType::Dir),
        models::FSObject::new(4, "4Xin", models::FSObjectType::File),
    ])

    }
    else {
    Json::from(vec![
        models::FSObject::new(12, "xiao", models::FSObjectType::Dir),
        models::FSObject::new(123, "bao", models::FSObjectType::Dir),
        models::FSObject::new(32, "bao", models::FSObjectType::Dir),
        models::FSObject::new(21, "Jiu", models::FSObjectType::File),
        models::FSObject::new(23,"Xin", models::FSObjectType::File),
    ])
}
}

#[post("/post/folder", data="<folder>")]
pub fn post_folder(folder: Json<models::FSFolder>){
    println!("Req for folder creation {:#?}", folder )
}

#[post("/post/file", data="<file>")]
pub fn post_file(file: Json<models::FSFile>){
    println!("Req for file creation {:#?}", file )
}
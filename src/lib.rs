use rocket::launch;
use rocket_cors::{AllowedOrigins, CorsOptions};
use teloxide::{Bot, dispatching::{Dispatcher, HandlerExt, UpdateFilterExt, dialogue::InMemStorage}, dptree, requests::{Requester, ResponseResult}, types::{CallbackQuery, Message, Update}};

mod commands;
mod storage;
mod server;


use crate::commands::{State};


pub async fn run_bot(){
    let bot=Bot::from_env();

    let mess_handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>,State>()
        .branch(dptree::case![State::Start].endpoint(commands::start))
        .endpoint(commands::unknown)
        // .branch(dptree::case![State::Command].filter_command::<Commands>()
        //                 .branch(dptree::case![Commands::SaveRecipe].endpoint(commands::save))
        //                 .branch(dptree::case![Commands::GetRecipe].endpoint(commands::get))
        // )
    ;
    let callback_handler = Update::filter_callback_query()
        .enter_dialogue::<CallbackQuery, InMemStorage<State>,State>()
        .branch(dptree::case![State::Start].endpoint(commands::start_callback))
        .branch(dptree::case![State::Get].endpoint(commands::get_callback))
        .branch(dptree::case![State::Save].endpoint(commands::save_callback))
    ;

    let handler = dptree::entry()
        .branch(mess_handler)
        .branch(callback_handler)
    ;

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

pub async fn run_server() -> Result<(), rocket::Error> {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::some_exact(&[
            "http://localhost:5173".to_string(), // URL вашего фронтенда
        ]))
        .allow_credentials(true)
        .to_cors()
        .expect("Ошибка при создании CORS Fairing");

    let rocket = rocket::build().mount("/", 
        rocket::routes![server::ping, server::get_fs]).attach(cors);
    rocket.launch().await?;
    Ok(())
}
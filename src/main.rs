use tokio;

use teloxide::{Bot, dispatching::{Dispatcher, HandlerExt, UpdateFilterExt, dialogue::InMemStorage}, dptree, requests::{Requester, ResponseResult}, types::{CallbackQuery, Message, Update}};

use crate::commands::{State};

mod commands;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    log::info!("starting recipe notes bot...");

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

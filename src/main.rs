use tokio;

use teloxide::{Bot, dispatching::{Dispatcher, HandlerExt, UpdateFilterExt, dialogue::InMemStorage}, dptree, requests::{Requester, ResponseResult}, types::{Message, Update}};

use crate::commands::{State, Commands, GetCommands};

mod commands;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    log::info!("starting recipe notes bot...");

    let bot=Bot::from_env();

    let handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>,State>()
        .branch(dptree::case![State::Start].endpoint(commands::start))
        .branch(dptree::case![State::Command].filter_command::<Commands>()
                        .branch(dptree::case![Commands::SaveRecipe].endpoint(commands::save))
                        .branch(dptree::case![Commands::GetRecipe].endpoint(commands::get))
        )
        .endpoint(commands::unknown)
    ;

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

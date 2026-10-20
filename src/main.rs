use std::thread::Builder;
use tokio;

use teloxide::{Bot, dispatching::{Dispatcher, UpdateFilterExt}, dptree, requests::{Requester, ResponseResult}, types::{Message, Update}};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    log::info!("starting recipe notes bot...");

    let bot=Bot::from_env();

    let handler = Update::filter_message()
        .endpoint(|bot: Bot, mes: Message| async move {
        bot.send_message(mes.chat.id, format!("Hello {}", mes.text().unwrap_or_default())).await?;
        ResponseResult::Ok(())
        }
    );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

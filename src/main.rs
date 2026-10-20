use std::thread::Builder;
use tokio;

use teloxide::{Bot, dptree, requests::Requester, types::Message};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    log::info!("starting recipe notes bot...");

    let bot=Bot::from_env();
    
    teloxide::repl(bot, |bot: Bot, mes: Message| async move {
        bot.send_message(mes.chat.id, format!("Hello {}", mes.text().unwrap_or_default())).await?;
        Ok(())
    }).await;
}

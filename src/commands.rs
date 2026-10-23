use std::error::Error;

use teloxide::{RequestError, dispatching::dialogue::{self, Dialogue, GetChatId, InMemStorage}, macros::BotCommands, payloads::SendMessageSetters, requests::{Requester, ResponseResult}, sugar::bot::BotMessagesExt, types::{CallbackQuery, ChatId, InlineKeyboardButton, InlineKeyboardMarkup, Message}};


#[derive(Clone,Default)]
pub enum State{
    #[default]
    Start,
    Get,
    Save
}

#[derive(Clone,Default)]
pub enum StateSave{
    #[default]
    SetRecipeName,
    SetRecipeTags,
    SetRecipeIngridients,
    SetRecipeNotes
}

pub fn start_commands_buttons() -> InlineKeyboardMarkup{
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Get recipe", "get")],
        vec![InlineKeyboardButton::callback("New recipe", "save")],
    ])
}

pub async fn start(bot:teloxide::Bot, mes: Message)->Result<(), Box<dyn std::error::Error + Send + Sync>>{
    menu(bot, mes.chat.id).await
}

pub async fn menu(bot:teloxide::Bot, id: ChatId)->Result<(), Box<dyn std::error::Error + Send + Sync>>{
    bot.send_message(id, "Hello").reply_markup(start_commands_buttons()).await?;
    Ok(())
}

pub fn get_commands_buttons() -> InlineKeyboardMarkup{
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Find by title", "get_by_title")],
        vec![InlineKeyboardButton::callback("Select by tags", "get_by_tags")],
        vec![InlineKeyboardButton::callback("Back to menu", "menu")],
    ])
}

pub fn menu_command_button() -> InlineKeyboardMarkup{
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Back to menu", "menu")],
    ])
}

pub async fn start_callback(bot:teloxide::Bot, dialogue: Dialogue<State, InMemStorage<State>>, q: CallbackQuery)->Result<(), Box<dyn std::error::Error + Send + Sync>>{
    // Обязательно отвечаем Telegram, чтобы убрать «часики» загрузки на кнопке
    bot.answer_callback_query(q.id.clone()).await?;

    if let  Some(data) = &q.data{
        let mes = q.regular_message().unwrap();
        
        bot.edit_reply_markup(mes).await?;
        
        match data.as_str(){
            "get" => {
                bot.send_message(q.chat_id().unwrap(), "Chose the way to get message.")
                    .reply_markup(get_commands_buttons()).await?;
                dialogue.update(State::Get).await?;
            },
            "save" => {
                bot.send_message(q.chat_id().unwrap(), "save selected")
                    .reply_markup(menu_command_button()).await?;
                dialogue.update(State::Save).await?;
            },
            _ => {},
        }
    }

    Ok(())
}

pub async fn get_callback(bot:teloxide::Bot, dialogue: Dialogue<State, InMemStorage<State>>, q: CallbackQuery)->Result<(), Box<dyn std::error::Error + Send + Sync>>{
    // Обязательно отвечаем Telegram, чтобы убрать «часики» загрузки на кнопке
    bot.answer_callback_query(q.id.clone()).await?;

    if let  Some(data) = &q.data{
        let mes = q.regular_message().unwrap();
        
        bot.edit_reply_markup(mes).await?;
        
        match data.as_str(){
            "menu" => {
                menu(bot, mes.chat.id).await?;
                dialogue.update(State::Start).await?;
            },
            _ => {},
        }
    }

    Ok(())
}

pub async fn save_callback(bot:teloxide::Bot, dialogue: Dialogue<State, InMemStorage<State>>, q: CallbackQuery)->Result<(), Box<dyn std::error::Error + Send + Sync>>{
    // Обязательно отвечаем Telegram, чтобы убрать «часики» загрузки на кнопке
    bot.answer_callback_query(q.id.clone()).await?;

    if let  Some(data) = &q.data{
        let mes = q.regular_message().unwrap();
        
        bot.edit_reply_markup(mes).await?;
        
        match data.as_str(){
            "menu" => {
                menu(bot, mes.chat.id).await?;
                dialogue.update(State::Start).await?;
            },
            _ => {},
        }
    }

    Ok(())
}

pub async fn save(bot:teloxide::Bot, dialogue: Dialogue<State, InMemStorage<State>>, mes: Message)->Result<(), Box<dyn std::error::Error + Send + Sync>>{
    bot.send_message(mes.chat.id, "Saved").await?;
    Ok(())
}

pub async fn get(bot:teloxide::Bot, dialogue: Dialogue<State, InMemStorage<State>>, mes: Message)->Result<(), Box<dyn std::error::Error + Send + Sync>>{
    bot.send_message(mes.chat.id, "Something...").await?;
    Ok(())
}

pub async fn unknown(bot:teloxide::Bot, mes: Message)->Result<(), Box<dyn Sync+Error+Send>>{
    bot.send_message(mes.chat.id, "unknown").await?;
    Ok(())
}
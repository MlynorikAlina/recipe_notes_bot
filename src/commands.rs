use std::error::Error;

use teloxide::{RequestError, dispatching::dialogue::{self, Dialogue, InMemStorage}, macros::BotCommands, requests::{Requester, ResponseResult}, types::Message};


#[derive(Clone,Default)]
pub enum State{
    #[default]
    Start,
    Command,
    Dialog
}

#[derive(Clone,Default)]
pub enum StateSave{
    #[default]
    SetRecipeName,
    SetRecipeTags,
    SetRecipeIngridients,
    SetRecipeNotes
}

#[derive(BotCommands, Clone)]
#[command(rename_rule="lowercase")]
pub enum Commands{
    #[command(aliases=["h","?"])]
    Help,
    #[command(aliases=["s","save"])]
    SaveRecipe,
    #[command(aliases=["g","get"])]
    GetRecipe,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule="lowercase")]
pub enum GetCommands{
    #[command(aliases=["search"])]
    GetByName,
    #[command(aliases=["tags"])]
    GetByTag,
    #[command(aliases=["list_all"])]
    ListAll,
}

pub async fn start(bot:teloxide::Bot, dialogue: Dialogue<State, InMemStorage<State>>, mes: Message)->Result<(), Box<dyn std::error::Error + Send + Sync>>{
    bot.send_message(mes.chat.id, "Hello").await?;
    dialogue.update(State::Command).await?;
    Ok(())
}

pub async fn save(bot:teloxide::Bot, dialogue: Dialogue<State, InMemStorage<State>>, mes: Message)->Result<(), Box<dyn std::error::Error + Send + Sync>>{
    bot.send_message(mes.chat.id, "Saved").await?;
    dialogue.update(State::Command).await?;
    Ok(())
}

pub async fn get(bot:teloxide::Bot, dialogue: Dialogue<State, InMemStorage<State>>, mes: Message)->Result<(), Box<dyn std::error::Error + Send + Sync>>{
    bot.send_message(mes.chat.id, "Something...").await?;
    dialogue.update(State::Command).await?;
    Ok(())
}

pub async fn unknown(bot:teloxide::Bot, mes: Message)->Result<(), Box<dyn Sync+Error+Send>>{
    bot.send_message(mes.chat.id, "unknown").await?;
    Ok(())
}
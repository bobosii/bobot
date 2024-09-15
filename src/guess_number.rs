use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, Message};
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct GameState {
    active_games: Mutex<HashMap<u64, i32>>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            active_games: Mutex::new(HashMap::new()),
        }
    }
}

/// Guess number between 1-100
#[poise::command(prefix_command)]
pub async fn guessnumber(ctx: Context<'_>) -> Result<(), Error> {
    let secret_number = rand::thread_rng().gen_range(1..100);

    ctx.say("Sayı tahmin etme oyunu başladı!!").await?;

    Ok(())
}

#[poise::command(prefix_command)]
pub async fn stopguess(ctx: Context<'_>) -> Result<(), Error> {
    //let mut data = ctx.data().game_state.active_games.lock().await;

   if data.remove(&ctx.author().id.into()).is_some() {
        ctx.say("Oyun sonlandırıldı").await?;
    } else {
        ctx.say("Oynanan sayı tahmin etme oyunu bulunmamakta")
            .await?;
    }

    Ok(())
}


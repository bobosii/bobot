use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use rand::Rng;
use std::cmp::Ordering;

/// Show this help menu
#[poise::command(prefix_command, track_edits)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "Slave of Ghost Village",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}
/// Guess a number !
///
/// Enter '!guessnumber 31' to guess number
#[poise::command(prefix_command)]
pub async fn guessnumber(
    ctx: Context<'_>,
    #[description = "What is your guess ?"] guess: String,
) -> Result<(), Error> {
    let secret_number: u32 = rand::thread_rng().gen_range(1..100);

    ctx.say("Sayi tahmin etme oyununa hos geldiniz !").await?;

    // Tahmini string olarak alıp sayıya dönüştürme
    let w_guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            ctx.say("Lutfen sadece sayi girin!").await?;
            return Ok(());
        }
    };

    println!("Secret Number: {:?}", secret_number);
    println!("Player Guess: {:?}", w_guess);

    // Tahminle gizli sayı karşılaştırılıyor
    match w_guess.cmp(&secret_number) {
        Ordering::Less => {
            ctx.say(format!(
                "Tahminiz çok düşük !, Tahmininiz: {:?}, Gizli sayı: {:?}",
                w_guess, secret_number
            ))
            .await?;
        }
        Ordering::Greater => {
            ctx.say(format!(
                "Tahminiz çok yüksek !, Tahmininiz: {:?} ,Gizli sayı: {:?}",
                w_guess, secret_number
            ))
            .await?;
        }
        Ordering::Equal => {
            ctx.say(format!(
                "Tebrikler kazandınız, Tahmininiz {:?}, Gizli sayı: {:?}",
                w_guess, secret_number
            ))
            .await?;
        }
    }

    Ok(())
}

/// Vote for something
///
/// Enter `!vote pumpkin` to vote for pumpkins
#[poise::command(prefix_command)]
pub async fn vote(
    ctx: Context<'_>,
    #[description = "What to vote for"] choice: String,
) -> Result<(), Error> {
    // Lock the Mutex in a block {} so the Mutex isn't locked across an await point
    let num_votes = {
        let mut hash_map = ctx.data().votes.lock().unwrap();
        let num_votes = hash_map.entry(choice.clone()).or_default();
        *num_votes += 1;
        *num_votes
    };

    let response = format!("Successfully voted for {choice}. {choice} now has {num_votes} votes!");
    ctx.say(response).await?;
    Ok(())
}

/// Retrieve number of votes
///
/// Retrieve the number of votes either in general, or for a specific choice:
/// ```
/// !getvotes
/// !getvotes pumpkin
/// ```
#[poise::command(prefix_command, track_edits, aliases("votes"))]
pub async fn getvotes(
    ctx: Context<'_>,
    #[description = "Choice to retrieve votes for"] choice: Option<String>,
) -> Result<(), Error> {
    if let Some(choice) = choice {
        let num_votes = *ctx.data().votes.lock().unwrap().get(&choice).unwrap_or(&0);
        let response = match num_votes {
            0 => format!("Nobody has voted for {} yet", choice),
            _ => format!("{} people have voted for {}", num_votes, choice),
        };
        ctx.say(response).await?;
    } else {
        let mut response = String::new();
        for (choice, num_votes) in ctx.data().votes.lock().unwrap().iter() {
            response += &format!("{}: {} votes", choice, num_votes);
        }

        if response.is_empty() {
            response += "Nobody has voted for anything yet :(";
        }

        ctx.say(response).await?;
    };

    Ok(())
}
/// Just saying Pong for Ping
/// !ping
#[poise::command(prefix_command)]
pub async fn ping(
    ctx: Context<'_>,
    #[description = "Pong?"] _say: Option<String>,
) -> Result<(), Error> {
    let response = "Pong?";
    ctx.reply(response).await?;
    Ok(())
}
/// Rock Paper Scissors
///
/// Type your choice !rockpaper rock
#[poise::command(prefix_command)]
pub async fn rockpaper(
    ctx: Context<'_>,
    #[description = "Rock Paper Scissors ?"] choice: String,
    user: Option<serenity::User>,
) -> Result<(), Error> {
    let game = ["rock", "paper", "scissors"];
    let user_choice = choice.trim().to_lowercase();
    let computer_cohice = game[rand::thread_rng().gen_range(1..3)];
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    match (user_choice.as_str(), computer_cohice) {
        ("rock", "scissors") | ("scissors", "paper") | ("paper", "rock") => {
            ctx.say(format!(
                "Bobot choice {:?}, Your choice {:?}",
                computer_cohice, user_choice
            ))
            .await?;
            ctx.say(format!("Winner is {}", u.name)).await?;
            ctx.say("Thanks for playing :)").await?;
            return Ok(());
        }
        ("paper", "scissors") | ("scissors", "rock") | ("rock", "paper") => {
            ctx.say(format!(
                "Bobot choice {:?}, Your choice {:?}",
                user_choice, computer_cohice
            ))
            .await?;
            ctx.say(format!("Winner is Bobot XD")).await?;
            ctx.say("Thanks for playing :)").await?;
            return Ok(());
        }
        _ => {
            ctx.say(format!(
                "Bobot choice {:?}, Your choice {:?}",
                user_choice, computer_cohice
            ))
            .await?;
            ctx.say("It's a tie!").await?;
            return Ok(());
        }
    }
}

/// See your account age
#[poise::command(prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.reply(response).await?;
    Ok(())
}

/// Source code of bobot
#[poise::command(prefix_command)]
pub async fn source(
    ctx: Context<'_>,
    #[description = "Shows bobot's githup repo"] _say: Option<String>,
) -> Result<(), Error> {
    let url = "https://github.com/bobosii/bobot";
    ctx.reply(url).await?;

    Ok(())
}

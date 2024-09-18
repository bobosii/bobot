use crate::{Context, Error};
use lazy_static::lazy_static;
use poise::serenity_prelude as serenity;
use rand::Rng;

lazy_static! {
    static ref WIN_GIFS: Vec<&'static str> = vec![
    "https://i.giphy.com/media/v1.Y2lkPTc5MGI3NjExcXNpdHE0MWh5Nzh6MjlvenN2ZmpzMzllYWNjM3B4aTY3bmF5bTBpbSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/MDJ9IbxxvDUQM/giphy.gif",
    "https://i.giphy.com/media/v1.Y2lkPTc5MGI3NjExajR6aXB5bmVscHh5YTlsaWZic3h1cGs4M2QzMTQxd3ppcTBvNDk4eiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/C9x8gX02SnMIoAClXa/giphy-downsized-large.gif",
        "https://i.giphy.com/media/v1.Y2lkPTc5MGI3NjExNm81cGVsODZnN2owZTVqeGduM3MwbDRkbjFyZDF6NzBlcGlscnZicSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/VOPK1BqsMEJRS/giphy.gif",
    ];
    static ref LOSE_GIFS: Vec<&'static str> = vec![
    "https://i.giphy.com/media/v1.Y2lkPTc5MGI3NjExZGZxeGJ5b3ppaWgwZmJiNzQ5dmE4Yjg2OWR6a2c5b3NhYjIybjU5NyZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/VbnUQpnihPSIgIXuZv/giphy.gif",
        "https://i.giphy.com/media/v1.Y2lkPTc5MGI3NjExeHJibTAwODNxMTRiMmplZHQwMTk2OWJzaGI3ZzBwYTY0aG93ejd6cSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/PDsgxQoXvUZGg/giphy.gif",
        "https://i.giphy.com/media/v1.Y2lkPTc5MGI3NjExdmw0dm85bDhxNXV3c2Y5aHFnYWZsNHpscHBzMXhpbnNrdDFsaHBvciZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/mIZ9rPeMKefm0/giphy.gif"
    ];
    static ref TIE_GIFS: Vec<&'static str> = vec![
        "https://media.giphy.com/media/dZf0umRo0UQbiAvsHF/giphy.gif",
        "https://media.giphy.com/media/4nSl9oAv5IYHy0eVB1/giphy.gif",
        "https://media.giphy.com/media/2yqyPZUR4mPFyRTpYi/giphy.gif",
        "https://media.giphy.com/media/j0gQA2VD38NKc9rc8y/giphy.gif",
    ];
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

    if !game.contains(&user_choice.as_str()) {
        ctx.say("Invalid choice! Please select Rock, Paper or Scissors !")
            .await?;
        return Ok(());
    }

    let result = match (user_choice.as_str(), computer_cohice) {
        ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => "win",
        ("rock", "paper") | ("paper", "scissors") | ("scissors", "rock") => "lose",
        _ => "tie",
    };

    ctx.say(format!(
        "Bobot chose {}, {} chose {}",
        computer_cohice, u.name, user_choice
    ))
    .await?;

    let gifs = match result {
        "win" => &*WIN_GIFS,
        "lose" => &*LOSE_GIFS,
        "tie" => &*TIE_GIFS,
        _ => return Ok(()),
    };
    let random_index = rand::thread_rng().gen_range(0..gifs.len());
    let selected_gif = gifs[random_index];
    match result {
        "win" => {
            ctx.say(format!("{} won \n{}", u.name, selected_gif))
                .await?;
        }
        "lose" => {
            ctx.say(format!("Bobot won \n{}", selected_gif)).await?;
        }
        "tie" => {
            ctx.say(format!("Draw \n{}", selected_gif)).await?;
        }
        _ => return Ok(()),
    }
    Ok(())
}

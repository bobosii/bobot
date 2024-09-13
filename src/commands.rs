use crate::{Context, Error};
use poise::serenity_prelude as serenity;

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
            extra_text_at_bottom:
                "Slave of Ghost Village",
            ..Default::default()
        },
    )
    .await?;
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

/// Write !age and see your account age

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

/// Source
///
/// bobot's source code link !source
#[poise::command(prefix_command)]
pub async fn source(
    ctx: Context<'_>,
    #[description = "Shows bobot's githup repo"] _say: Option<String>,
) -> Result<(), Error> {
    let url = "https://github.com/bobosii/bobobot";
    ctx.reply(url).await?;

    Ok(())
}

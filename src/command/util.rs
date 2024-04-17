use poise::serenity_prelude as serenity;

use crate::Context;
use crate::Error;

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    info!("Executed command `age` successfully");
    Ok(())
}

/// Show information about this bot
#[poise::command(slash_command)]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!(
        "DeerBot ReBleated v{} was created by Shibe Drill (@shibedrill) using Rust and Poise.\nVisit her website: https://riverdev.carrd.co\nCheck out her Github: https://github.com/shibedrill\nPoise: https://docs.rs/poise/latest/poise/\nRust: https://www.rust-lang.org/",
        env!("CARGO_PKG_VERSION")
    ))
    .await?;
    info!("Executed command `info` successfully");
    Ok(())
}

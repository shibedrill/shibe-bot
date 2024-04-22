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

/// Add information to the shared settings
#[poise::command(slash_command)]
pub async fn add_channel(
    ctx: Context<'_>,
    #[description = "Selected channel"] channel: Option<serenity::Channel>,
) -> Result<(), Error> {
    if let Some(channel_ok) = channel {
        let config = &mut ctx.data().config_manager.lock().await;
        let channel_id = { u64::from(channel_ok.id()) };
        config.channels.push(channel_id);
        config.store().unwrap();
        ctx.say(format!(
            "Successfully added <#{}> to the channel registry.",
            channel_id
        ))
        .await?;
        info!("Executed command `add_channel` successfully");
    } else {
        ctx.say("Channel with supplied ID was not found.").await?;
        error!("Failed to execute command `add_channel`.");
    }
    Ok(())
}

#[poise::command(slash_command)]
pub async fn list_channels(ctx: Context<'_>) -> Result<(), Error> {
    let config = &mut ctx.data().config_manager.lock().await;
    ctx.say(format!(
        "Current channel IDs in registry: {:#?}",
        config.channels
    ))
    .await?;
    info!("Executed command `list_channels` successfully");
    Ok(())
}

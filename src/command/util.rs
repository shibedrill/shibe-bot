use poise::serenity_prelude as serenity;
use rand::Rng;

use crate::Context;
use crate::Error;

use build_time::build_time_local;

const INVITE_LINK: &str = "https://discord.com/oauth2/authorize?client_id=1030701552941412382&permissions=116736&response_type=code&redirect_uri=https%3A%2F%2Fdiscordapp.com%2Foauth2%2Fauthorize%3F%26client_id%3D1030701552941412382%26scope%3Dbot&scope=guilds+bot";

/// Add this bot to your server
#[poise::command(slash_command)]
pub async fn invite(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say(format!(
        "To add me to your server, click [this link]({INVITE_LINK}), or open it in the \
        browser and enable all the requested permissions. Then select your \
        server to add it.",
    ))
    .await?;
    info!("Executed command `invite` successfully");
    Ok(())
}

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
#[poise::command(slash_command, global_cooldown = 30)]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!(
        "Shibe Bot v{} was created by Shibe Drill (@shibedrill) \
        using Rust and Poise.\n\
        rustc version: {}\n\
        Build timestamp: {}\n\
        Website: <https://riverdev.carrd.co>\n\
        Source code: <https://github.com/shibedrill/shibe-bot>\n\
        Poise: <https://docs.rs/poise/latest/poise/>\n\
        Rust: <https://www.rust-lang.org/>",
        env!("CARGO_PKG_VERSION"),
        rustc_version_runtime::version(),
        build_time_local!()
    ))
    .await?;
    info!("Executed command `info` successfully");
    Ok(())
}

/// Add channel to the registry
#[poise::command(slash_command, required_permissions = "MANAGE_CHANNELS")]
pub async fn add_channel(
    ctx: Context<'_>,
    #[description = "Selected channel"] channel: serenity::Channel,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let config = &mut ctx.data().config_manager.lock().await;
    let channel_id = { u64::from(channel.id()) };
    match config.channels.iter().find(|item| **item == channel_id) {
        None => {
            config.channels.push(channel_id);
            ctx.say(format!("Successfully added <#{channel_id}> to the channel registry."))
            .await?;
        }
        Some(_) => {
            ctx.say(format!("Channel <#{channel_id}> is already in registry."))
                .await?;
        }
    }
    config.store().expect("Unable to store config");
    info!("Executed command `add_channel` successfully");
    Ok(())
}

/// Remove channel from the registry
#[poise::command(slash_command, required_permissions = "MANAGE_CHANNELS")]
pub async fn remove_channel(
    ctx: Context<'_>,
    #[description = "Selected channel"] channel: serenity::Channel,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let config = &mut ctx.data().config_manager.lock().await;
    let channel_id = { u64::from(channel.id()) };
    match config.channels.iter().position(|item| *item == channel_id) {
        None => {
            ctx.say(format!("Channel <#{channel_id}> was not in the channel registry."))
            .await?;
        }
        Some(found) => {
            config.channels.remove(found);
            ctx.say(format!("Successfully removed <#{channel_id}> from the channel registry."))
            .await?;
        }
    }
    config.store().expect("Unable to store config");
    info!("Executed command `remove_channel` successfully");
    Ok(())
}

/// List channels held in the registry
#[poise::command(slash_command, required_permissions = "MANAGE_CHANNELS")]
pub async fn list_channels(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let config = &mut ctx.data().config_manager.lock().await;
    ctx.say(format!(
        "Current channel IDs in registry: \n{:#?}",
        config.channels
    ))
    .await?;
    info!("Executed command `list_channels` successfully");
    Ok(())
}

/// Roll a dice with a certain amount of sides, 2 sides is a coin toss
#[poise::command(slash_command)]
pub async fn dice(
    ctx: Context<'_>,
    #[description = "The amount of sides on the dice"] sides: u32,
) -> Result<(), Error> {
    let answer: u32 = {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=sides)
    };
    let response = match sides {
        0 | 1 => {
            ctx.defer_ephemeral().await?;
            String::from("You cannot roll a dice with 0 or 1 sides.")
        }
        2 => {
            format!(
                "Coin toss landed on: {}",
                match answer {
                    1 => "heads",
                    2 => "tails",
                    _ => unreachable!(),
                }
            )
        }
        _ => format!("Rolled a random number from 1 to {sides}, got: {answer}")
    };
    ctx.say(response).await?;
    info!("Executed command `dice` successfully");
    Ok(())
}

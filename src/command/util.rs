use poise::serenity_prelude as serenity;
use rand::Rng;

use crate::Context;
use crate::Error;

const INVITE_LINK: &str = "https://discord.com/oauth2/authorize?client_id=1030701552941412382&permissions=116736&response_type=code&redirect_uri=https%3A%2F%2Fdiscordapp.com%2Foauth2%2Fauthorize%3F%26client_id%3D1030701552941412382%26scope%3Dbot&scope=guilds+bot";

/// Add this bot to your server
#[poise::command(slash_command)]
pub async fn invite(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say(format!(
        "To add me to your server, click [this link]({}) and enable all the requested permissions.",
        INVITE_LINK
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
#[poise::command(slash_command)]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!(
        "Shibe Bot v{} was created by Shibe Drill (@shibedrill) using Rust and Poise.\nVisit her website: https://riverdev.carrd.co\nCheck out her Github: https://github.com/shibedrill\nPoise: https://docs.rs/poise/latest/poise/\nRust: https://www.rust-lang.org/",
        env!("CARGO_PKG_VERSION")
    ))
    .await?;
    info!("Executed command `info` successfully");
    Ok(())
}

/// Add channel to the registry
#[poise::command(slash_command)]
pub async fn add_channel(
    ctx: Context<'_>,
    #[description = "Selected channel"] channel: Option<serenity::Channel>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    if let Some(channel_ok) = channel {
        let config = &mut ctx.data().config_manager.lock().await;
        let channel_id = { u64::from(channel_ok.id()) };
        config.channels.push(channel_ok);
        config.store().unwrap();
        ctx.say(format!(
            "Successfully added <#{}> to the channel registry.",
            channel_id
        ))
        .await?;
        info!("Executed command `add_channel` successfully");
    } else {
        ctx.say("Channel with supplied ID was not found.").await?;
        error!("Failed to execute command `add_channel`");
    }
    Ok(())
}

/// Remove channel from the registry
#[poise::command(slash_command)]
pub async fn remove_channel(
    ctx: Context<'_>,
    #[description = "Selected channel"] channel: Option<serenity::Channel>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    if let Some(channel_ok) = channel {
        let config = &mut ctx.data().config_manager.lock().await;
        let channel_id = { u64::from(channel_ok.id()) };
        config.channels.retain(|c| c.id() != channel_ok.id());
        config.store().unwrap();
        ctx.say(format!(
            "Successfully removed <#{}> from the channel registry.",
            channel_id
        ))
        .await?;
        info!("Executed command `remove_channel` successfully");
    } else {
        ctx.say("Channel with supplied ID was not found.").await?;
        error!("Failed to execute command `remove_channel`.");
    }
    Ok(())
}

/// List channels held in the registry
#[poise::command(slash_command)]
pub async fn list_channels(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let config = &mut ctx.data().config_manager.lock().await;
    let mut channel_ids: Vec<u64> = vec![];
    config
        .channels
        .iter()
        .for_each(|c| channel_ids.push(u64::from(c.id())));
    ctx.say(format!(
        "Current channel IDs in registry: \n{:#?}",
        channel_ids
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
        _ => {
            format!(
                "Rolled a random number from 1 to {}, got: {}",
                sides, answer
            )
        }
    };
    ctx.say(response).await?;
    info!("Executed command `dice` successfully");
    Ok(())
}

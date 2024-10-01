use build_time::build_time_local;

use crate::Context;
use crate::Error;

/// Update the bot remotely (Requires updater systemd service)
#[poise::command(slash_command, owners_only, hide_in_help)]
pub async fn update(ctx: Context<'_>) -> Result<(), Error> {
    let command_result = std::process::Command::new("systemd")
        .arg("--user")
        .arg("restart")
        .arg("shibe-bot-update.service")
        .spawn();
    match command_result {
        Ok(_child) => {
            ctx.say(format!(
                "Initialized restart service successfully.\n\
            Expect brief outage soon.\n\
            Current version: {}\n\
            Timestamp of last build: {}",
                env!("CARGO_PKG_VERSION"),
                build_time_local!()
            ))
            .await?;
            info!("Initialized restart service successfully");
        }
        Err(what) => {
            ctx.say(format!(
                "Failed to initialize restart service. Reason: {}",
                what
            ))
            .await?;
            error!("Failed to initialize restart service: {}", what);
        }
    }
    Ok(())
}

/// Shut down the bot remotely
#[poise::command(slash_command, owners_only, hide_in_help)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Shutting down...").await?;
    ctx.framework().shard_manager().shutdown_all().await;
    info!("Executed command `shutdown` successfully");
    Ok(())
}

/// Restart the bot remotely
#[poise::command(slash_command, owners_only, hide_in_help)]
pub async fn restart(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Restarting...").await?;
    for shard in ctx.framework().shard_manager().shards_instantiated().await {
        ctx.framework().shard_manager().restart(shard).await;
    }
    info!("Executed command `restart` successfully");
    Ok(())
}

/// Say a specific message
#[poise::command(slash_command, owners_only, hide_in_help)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "The message content to send"] what: String,
    #[description = "Whether to make it ephemeral"] ephemeral: Option<bool>,
) -> Result<(), Error> {
    if ephemeral == Some(true) {
        ctx.defer_ephemeral().await?;
    }
    ctx.say(what).await?;
    Ok(())
}

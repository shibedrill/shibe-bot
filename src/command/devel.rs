use crate::Context;
use crate::Error;

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

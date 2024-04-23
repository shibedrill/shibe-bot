use crate::Context;
use crate::Error;

#[poise::command(slash_command, owners_only, hide_in_help)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say("Shutting down...").await?;
    info!("Received `shutdown` command, shutting down all shards");
    ctx.framework().shard_manager().shutdown_all().await;
    Ok(())
}

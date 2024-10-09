use crate::Context;
use crate::Error;

/// Print version and build information
#[poise::command(slash_command)]
pub async fn version(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!(
        "Source:\n\
        \tPackage version: {}\n\
        \tCommit ID: {}\n\
        \tCommit date: {}\n\
        \tCommit author: {} ({})\n\
        \tCommit message: {}\n\
        Build:\n\
        \tBuild date: {}\n\
        \tBuild timestamp: {}\n\
        \tTarget triple: {}\n\
        \trustc version: {}\n",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_COMMIT_ID"),
        env!("GIT_COMMIT_DATE"),
        env!("GIT_COMMIT_AUTHOR_NAME"),
        env!("GIT_COMMIT_AUTHOR_EMAIL"),
        env!("GIT_COMMIT_MSG"),
        env!("VERGEN_BUILD_DATE"),
        env!("VERGEN_BUILD_TIMESTAMP"),
        env!("VERGEN_CARGO_TARGET_TRIPLE"),
        env!("VERGEN_RUSTC_SEMVER"),
    ))
    .await?;
    Ok(())
}

/// Update the bot remotely (Requires updater systemd service)
#[poise::command(slash_command, owners_only, hide_in_help)]
pub async fn update(ctx: Context<'_>) -> Result<(), Error> {
    let command_result = std::process::Command::new("systemctl")
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
                env!("VERGEN_BUILD_TIMESTAMP")
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

use std::convert::Infallible;
use std::os::unix::process::CommandExt;

use crate::Context;
use crate::Error;

use octocrab;
use std::io::Write;

use self_replace;
use zip;

use minreq;

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
        env!("GIT_COMMIT_ID_SHORT"),
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
pub async fn update(ctx: Context<'_>, override_check: bool) -> Result<(), Error> {
    // Check if the current commit hash is different from HEAD
    let head: octocrab::models::repos::Ref = octocrab::instance()
        .get(
            "/repos/shibedrill/shibe-bot/git/refs/heads/main",
            None::<&octocrab::models::Repository>,
        )
        .await?;
    if let octocrab::models::repos::Object::Commit { sha, url: _ } = head.object {
        if sha == env!("GIT_COMMIT_ID") {
            if override_check {
                info!("Update unnecessary, but check overridden.");
                ctx.say("Update unecessary, but check overridden. Updating.").await?;
                let Err(what) = self_update();
                error!("Update failed: {}", what);
                ctx.say(format!("Error occurred while updating: {}", what))
                    .await?;
            } else {
                info!("Update unnecessary: Commit ID of remote is same as compiled commit.");
                ctx.say("Update unnecessary.").await?;
            }
        } else {
            info!("Update required, latest commit hash: {}", sha);
            let Err(what) = self_update();
            error!("Update failed: {}", what);
            ctx.say(format!("Error occurred while updating: {}", what))
                .await?;
        }
    } else {
        ctx.say("Update failed: Object field in response is not a Commit.").await?;
        error!("Checking for updates failed: Response field incorrect type");
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

fn self_update() -> Result<Infallible, Error> {
    let artifact_url = "https://nightly.link/shibedrill/shibe-bot/workflows/rust/main/artifact.zip";
    let tempdir = tempfile::Builder::new().prefix("shibe-bot").tempdir()?;
    trace!("Created tempdir successfully: {}", tempdir.path().display());
    let response = minreq::get(artifact_url).send()?;

    let mut dest = std::fs::File::create_new(tempdir.path().join("artifact.zip"))?;
    let content = response.as_bytes();
    dest.write_all(&content)?;
    trace!("Downloaded latest build artifact successfully");

    
    let mut archive = zip::ZipArchive::new(dest)?;
    trace!("Created zip archive reader");
    let mut zipped_bin = archive.by_index(0)?;
    let new_bin_path = tempdir.path().join("shibe-bot");
    let mut new_bin = std::fs::File::create_new(&new_bin_path)?;
    trace!("Created new file for binary");

    std::io::copy(&mut zipped_bin, &mut new_bin)?;
    trace!("Extracted binary successfully");

    self_replace::self_replace(&new_bin_path)?;
    trace!("Replaced self with new binary successfully");

    let new_command_args: Vec<_> = std::env::args_os().skip(1).collect();
    let new_command_path = std::env::current_exe()?;
    trace!("Got current executable path successfully: {}", new_command_path.display());

    Err(Box::new(
        std::process::Command::new(new_command_path)
            .args(&new_command_args)
            .exec(),
    ))
}

mod test {

    #[cfg(test)]
    use std::convert::Infallible;
    #[cfg(test)]
    use crate::Error;

    #[test]
    fn test_self_update() -> Result<Infallible, Error> {
        use pretty_env_logger;
        use crate::command::devel::self_update;
        pretty_env_logger::init();
        self_update()
    }

}

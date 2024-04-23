use crate::Context;
use crate::Error;

use poise::serenity_prelude as serenity;
use rand::prelude::SliceRandom;
use rand::*;

/// mrow
#[poise::command(slash_command)]
pub async fn meow(ctx: Context<'_>) -> Result<(), Error> {
    let meows = [
        "meow",
        "mrow",
        "mrrrp",
        "mraaw",
        "bwrrrr",
        "mrrghh",
        "mrowwwwwwwwwwww",
        "FUCK",
    ];
    let response = {
        let mut rng = rand::thread_rng();
        match rng.gen_bool(0.1) {
            true => "woof",
            // Will never return None. The source is staticaly defined.
            // We know it will always have items in it.
            false => meows.choose(&mut rng).unwrap(),
        }
    };
    ctx.say(response).await?;
    info!("Executed command `meow` successfully");
    Ok(())
}

/// penis
#[poise::command(slash_command)]
pub async fn penis(
    ctx: Context<'_>,
    #[description = "The target user"] target: Option<serenity::User>,
) -> Result<(), Error> {
    if let Some(target_unwrapped) = target {
        ctx.say(format!(
            "<@{}> has been penised. they must penis another user to become un-penised",
            target_unwrapped.id
        ))
        .await?;
        info!("Executed command `penis` successfully");
    } else {
        error!("Failed to execute command `penis`");
    }
    Ok(())
}

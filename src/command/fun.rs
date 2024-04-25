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
    #[description = "The target user"] target: serenity::User,
) -> Result<(), Error> {
    ctx.say(format!(
        "<@{}> has been penised. they must penis another user to become un-penised",
        target.id
    ))
    .await?;
    info!("Executed command `penis` successfully");
    Ok(())
}

/// Magic 8-ball
#[poise::command(slash_command)]
pub async fn eightball(ctx: Context<'_>) -> Result<(), Error> {
    let responses = [
    "It is certain",
    "It is decidedly so",
    "Without a doubt",
    "Yes definitely",
    "You may rely on it",
    "As I see it, yes",
    "Most likely",
    "Outlook good",
    "Yes",
    "Signs point to yes",
    "Reply hazy, try again",
    "Ask again later",
    "Better not to tell you now",
    "Cannot predict now",
    "Concentrate and ask again",
    "Don't count on it",
    "My reply is no",
    "My sources say no",
    "Outlook not so good",
    "Very doubtful",
    ];
    let response = {
        let mut rng = rand::thread_rng();
        responses.choose(&mut rng).unwrap()
    };
    ctx.say(format!("Magic 8-ball says: '{}'", *response))
    .await?;
    info!("Executed command `eightball` successfully");
    Ok(())
}

/// BITE BITE BITE
#[poise::command(slash_command)]
pub async fn bite(
    ctx: Context<'_>,
    #[description = "The target user"] target: serenity::User,
) -> Result<(), Error> {
    ctx.say(format!(
        "<@{}> has been bitten by <@{}>",
        target.id,
        ctx.author().id,
    ))
    .await?;
    info!("Executed command `bite` successfully");
    Ok(())
}

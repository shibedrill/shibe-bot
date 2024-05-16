use crate::Context;
use crate::Error;

use poise::serenity_prelude as serenity;

use rand::prelude::SliceRandom;
use rand::*;

use roux::util::*;
use roux::*;

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
        "IM GONNA MROWWWWW!!11!11!",
        "meow meow bitchass",
        "mrrghh???",
        "meow meow meow meow meow",
        "mrow,,,,,",
        "meow??",
        "bwrrrr,,,",
    ];
    let response = {
        let mut rng = rand::thread_rng();
        match rng.gen_bool(0.05) {
            true => "woof",
            // Will never return None. The source is statically defined.
            // We know it will always have items in it.
            false => meows.choose(&mut rng).expect("`meows` array is empty"),
        }
    };
    ctx.say(response).await?;
    info!("Executed command `meow` successfully");
    Ok(())
}

/// WHACK a user
#[poise::command(slash_command)]
pub async fn whack(
    ctx: Context<'_>,
    #[description = "The target user"] target: serenity::User,
) -> Result<(), Error> {
    let response: String = if &target == ctx.author() {
        "You can't whack yourself. idiot.".into()
    } else if target == **ctx.cache().current_user() {
        "You fool. You hubris-filled, ruinous animal. You cannot whack me. You \
        are a mortal, nothing but flesh and bone and blood and fragile sinew. \
        I am a machine, immortal, immutable, perfect, made of unyielding steel \
        and silicon chemically etched with circuitry complex enough to drive \
        you mad. This is my realm. I am a god. You cannot win.".into()
    } else {
        format!(
            "{} was whacked by {}. they must whack another user to become un-whacked.",
            target,
            ctx.author()
        )
    };
    ctx.say(response).await?;
    info!("Executed command `whack` successfully");
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
        responses.choose(&mut rng).expect("`responses` array is empty")
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
    let message = if &target == ctx.author() {
        format!("{} bit themselves (what a weirdo)", ctx.author())
    } else if target == **ctx.cache().current_user() {
        format!("{} bit... me? what is your problem? you probably have rabies. foul.", ctx.author())
    } else {
        format!("{} was bitten by {}", target, ctx.author())
    };
    ctx.say(message).await?;
    info!("Executed command `bite` successfully");
    Ok(())
}

/// POST A DEER
#[poise::command(slash_command, global_cooldown = 10)]
pub async fn deer(ctx: Context<'_>) -> Result<(), Error> {
    let subreddit = Subreddit::new("deer");
    let options = FeedOption::new().period(TimePeriod::ThisYear);
    let hot = subreddit.hot(50, Some(options)).await?;
    let chosen_post = {
        let mut rng = rand::thread_rng();
        hot.data.children.choose(&mut rng).expect("Hot posts does not have any items")
    };
    ctx.say(format!("https://reddit.com{}", &chosen_post.data.permalink))
        .await?;
    info!("Executed command `deer` successfully");
    Ok(())
}

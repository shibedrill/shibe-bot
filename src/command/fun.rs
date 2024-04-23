use crate::Context;
use crate::Error;

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
        "waoaugh,,,,",
    ];
    let response = {
        let mut rng = rand::thread_rng();
        match rng.gen_bool(0.1) {
            true => "woof",
            false => meows.choose(&mut rng).unwrap(),
        }
    };
    ctx.say(response).await?;
    info!("Executed command `meow` successfully");
    Ok(())
}

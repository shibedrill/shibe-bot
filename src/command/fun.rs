
use crate::Context;
use crate::Error;

/// MAKE HER BLEAT
#[poise::command(slash_command)]
pub async fn bleat(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("BLEAT TEST")
    .await?;
    info!("Executed command `bleat` successfully");
    Ok(())
}

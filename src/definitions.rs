// Tokio async crap

use poise::serenity_prelude as serenity;

#[allow(unused_imports)]
use crate::settings::*;

// Data passed to every command (shared state)
pub struct Data {

}

// Errors returnable by a command
pub type Error = Box<dyn std::error::Error + Send + Sync>;

// The full context passed to a command
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn event_handler(
    _ctx: &serenity::Context,
    _event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {

    // Future event handling will go here
    // Data will contain the database connection

    Ok(())
}
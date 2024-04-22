
use std::sync::{Arc};
use tokio::sync::Mutex;

use dotenv::dotenv;

use poise::serenity_prelude as serenity;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use serde::*;

mod command;
use crate::{command::{fun::*, util::*}, settings::SettingsManager};

mod settings;

struct Data {
    config_manager: Arc<Mutex<SettingsManager<Settings>>>
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    channels: Vec<u64>,
}

#[tokio::main]
async fn main() {
    // Get secure env vars from .env file
    dotenv().ok();
    // Get token from environment
    let token = std::env::var("TOKEN").expect("Getting TOKEN from environment failed");
    // Set up some non privileged intents
    let intents = serenity::GatewayIntents::non_privileged();

    // Initialize logging
    pretty_env_logger::init();

    // Configure persistent options
    let config = Settings { channels: vec![] };
    let config_manager = Arc::new(Mutex::new(SettingsManager::manage("settings.json", config)));
    config_manager.lock().await.update();

    // Set up framework
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), info(), bleat(), add_channel(), list_channels()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {config_manager})
            })
        })
        .build();

    // Log pertinent info
    info!("Built framework successfully");
    {
        let mut commands: Vec<&str> = vec![];
        framework
            .options()
            .commands
            .iter()
            .for_each(|c| commands.push(&c.name));
        info!("Registered commands: {:?}", commands);
    }

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    info!("Built client successfully");
    info!("Starting client");
    client.unwrap().start().await.unwrap();
}

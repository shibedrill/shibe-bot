#![forbid(unsafe_code)]

// Tokio async crap
use poise::serenity_prelude::FullEvent;
use std::sync::Arc;
use tokio::sync::Mutex;

// For secure credential handling
use dotenvy::dotenv;

// Poise and Serenity - Framework and API prelude
use poise::serenity_prelude as serenity;

// Logging stuff
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

// For managing config storage
use serde::*;
mod settings;
use crate::settings::*;

// Bot commands
mod command;
use crate::command::{
    // Commands for development and testing
    devel::*,
    // Fun!!!
    fun::*,
    // Useful commands for mods
    util::*,
};

// Data passed to every command (shared state)
struct Data {
    config_manager: Arc<Mutex<SettingsManager<Settings>>>,
}

// Errors returnable by a command
type Error = Box<dyn std::error::Error + Send + Sync>;

// The full context passed to a command
type Context<'a> = poise::Context<'a, Data, Error>;

// The structure making up the configuration
#[derive(Debug, Serialize, Deserialize, Default)]
struct Settings {
    channels: Vec<u64>,
}

// Path at which our settings are stored (currently PWD)
const SETTINGS_PATH: &str = "settings.json";

async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    if let FullEvent::ChannelDelete {
        channel,
        messages: _,
    } = event
    {
        info!("Handling event type: ChannelDelete({})", channel.id);
        data.config_manager
            .lock()
            .await
            .channels
            .retain(|item| *item != u64::from(channel.id));
    }
    Ok(())
}

// Main function for setup
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
    let config_manager: Arc<Mutex<SettingsManager<Settings>>> = Arc::new(Mutex::new(
        SettingsManager::load(SETTINGS_PATH)
            .unwrap_or(SettingsManager::manage(SETTINGS_PATH, Settings::default())),
    ));
    config_manager.lock().await.store();

    // Set up framework
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // +---------------------------------------------------------+
            // |                    ADD COMMANDS HERE                    |
            // +---------------------------------------------------------+
            commands: vec![
                // Util
                age(),
                info(),
                add_channel(),
                remove_channel(),
                list_channels(),
                invite(),
                dice(),
                // Dev
                shutdown(),
                restart(),
                say(),
                // Fun
                meow(),
                whack(),
                eightball(),
                bite(),
                deer(),
            ],
            initialize_owners: true,
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                // Shared data has to go here!!!
                Ok(Data { config_manager })
            })
        })
        .build();

    // Log pertinent info
    info!("Built framework successfully");
    {
        // List registered commands
        let mut commands: Vec<&str> = vec![];
        framework
            .options()
            .commands
            .iter()
            .for_each(|c| commands.push(&c.name));
        info!("Registered commands: {:?}", commands);
    }

    // Build client
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap_or_else(|e| {
            error!("Building client failed: {}", e);
            std::process::exit(-1);
        });
    info!("Built client successfully");

    // List the owner
    info!(
        "Registered owner: {:?}",
        client
            .http
            .get_current_application_info()
            .await
            .unwrap_or_else(|e| {
                error!("Getting application info failed: {}", e);
                std::process::exit(-1);
            })
            .owner
            .unwrap_or_else(|| {
                error!("Getting owner info failed: `.owner` is `None`");
                std::process::exit(-1);
            })
            .name
    );

    // Finally start everything. Nothing after this should be reachable normally.
    info!("Starting client");
    client.start().await.unwrap_or_else(|e| {
        error!("Starting client failed: {}", e);
        std::process::exit(-1);
    });
    info!("All tasks finished, shutting down");
}

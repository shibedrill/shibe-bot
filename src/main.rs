#![forbid(unsafe_code)]

use std::env;

// For secure credential handling
use dotenvy::dotenv;

use poise::serenity_prelude::ActivityData;
// Poise and Serenity - Framework and API prelude
use poise::serenity_prelude as serenity;

// Logging stuff
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

// Definitions
mod definitions;
use crate::definitions::event_handler;
use crate::definitions::*;

// Settings manager
mod settings;
#[allow(unused_imports)]
use crate::settings::*;

// Schema in preparation for database
mod schema;
#[allow(unused_imports)]
use crate::schema::*;

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

// Path at which our settings are stored (currently PWD)
//const SETTINGS_PATH: &str = "settings.json";

// Main function for setup
#[tokio::main]
async fn main() {
    // Initialize logging
    pretty_env_logger::init();
    info!("Initialized logger successfully");
    match env::current_exe() {
        Ok(exe) => info!("Got current exe successfully: {}", exe.display()),
        Err(err) => error!("Failed to get exe: {}", err),
    }
    // Get secure env vars from .env file
    match dotenv() {
        Ok(_) => info!("Loaded env vars from .env successfully"),
        Err(e) => error!("Failed to get vars from .env: {}", e),
    }

    // Get token from environment
    let token = std::env::var("TOKEN")
        .inspect_err(|e| {
            error!("Failed to get TOKEN from environment: {}", e);
        })
        .expect("Failed to get TOKEN from environment");
    info!("Got TOKEN successfully");
    // Set up some non privileged intents
    let intents = serenity::GatewayIntents::non_privileged();

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
                invite(),
                dice(),
                // Dev
                shutdown(),
                restart(),
                say(),
                update(),
                version(),
                // Fun
                meow(),
                whack(),
                eightball(),
                bite(),
                deer(),
                curbstomp(),
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
                Ok(Data {})
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
        .activity(ActivityData::custom(format!(
            "Version {}!",
            env!("CARGO_PKG_VERSION")
        )))
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

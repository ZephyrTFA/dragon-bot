use std::env;

use log::{error, info, warn};
use module::event_handler::ModuleEventHandler;
use serenity::{Client, all::GatewayIntents};
use tokio::main;

pub mod module;

#[main]
async fn main() {
    env_logger::init();

    let data_path = env::var("DATA_PATH");
    let data_path = if data_path.is_err() {
        warn!("DATA_PATH is not set, defaulting to ``");
        "".to_string()
    } else {
        data_path.unwrap()
    };

    let token = env::var("DISCORD_TOKEN");
    if token.is_err() {
        error!("DISCORD_TOKEN not set.");
        return;
    }
    let token = token.unwrap();

    let client = Client::builder(&token, GatewayIntents::all())
        .event_handler(ModuleEventHandler {})
        .await;

    if client.is_err() {
        let error = unsafe { client.unwrap_err_unchecked() }.to_string();
        error!("failed to create discord client: {error}");
        return;
    }

    let run_result = client.unwrap().start_autosharded().await;
    if run_result.is_err() {
        error!("failed to run discord client: {}", run_result.unwrap_err());
        return;
    }

    info!("client exited.");
}

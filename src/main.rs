use std::{
    env,
    fs::{create_dir_all, exists},
    process::exit,
};

use log::{error, info, warn};
use module::event_handler::ModuleEventHandler;
use serenity::{Client, all::GatewayIntents};
use tokio::{main, sync::OnceCell};

pub mod module;

pub fn data_path() -> &'static String {
    if !DATA_PATH.initialized() {
        DATA_PATH
            .set(if let Ok(data_path) = env::var("DATA_PATH") {
                data_path
            } else {
                warn!("DATA_PATH not set, defaulting to local data directory.");

                dirs::data_dir()
                    .or_else(|| {
                        error!("failed to locate data directory");
                        exit(1);
                    })
                    .unwrap()
                    .join("dragon-bot")
                    .display()
                    .to_string()
            })
            .unwrap_or_else(|e| {
                error!("failed to set data path static cell: {e}");
                exit(1);
            });
        info!("Data Directory: {:?}", DATA_PATH.get().unwrap());

        let data_path = DATA_PATH.get().unwrap();
        if exists(data_path).is_err() {
            if let Err(error) = create_dir_all(data_path) {
                error!("failed to create data directory: {error}");
                exit(1);
            }
        }
    }

    DATA_PATH.get().unwrap()
}
static DATA_PATH: OnceCell<String> = OnceCell::const_new();

#[main]
async fn main() {
    env_logger::init();
    data_path();

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

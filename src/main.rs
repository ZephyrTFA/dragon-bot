use chrono::Utc;
use core::event_handler::ModuleEventHandler;
use fern::colors::{Color, ColoredLevelConfig};
use log::{LevelFilter, error, info};
use serenity::{Client, all::GatewayIntents};
use std::env;
use tokio::main;
use util::data_path;

pub mod core;
pub mod module;
pub mod util;

#[main]
async fn main() {
    let fern_colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::Blue);
    fern::Dispatch::new()
        .level(LevelFilter::Info)
        .level_for("dragon_bot", LevelFilter::Debug)
        .level_for("tracing", LevelFilter::Off)
        .level_for("serenity", LevelFilter::Warn)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {message}",
                Utc::now(),
                fern_colors.color(record.level()),
                record.target().split("::").next().unwrap()
            ))
        })
        .chain(std::io::stdout())
        .apply()
        .expect("failed to set fern as logger");

    data_path().await.expect("failed to init data path");

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

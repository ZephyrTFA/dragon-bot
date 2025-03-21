use crate::module::{commands::CommandError, config::ConfigError, errors::ModuleError};
use log::{debug, error, info, warn};
use serenity::{
    all::{CacheHttp, Context, GuildId, GuildInfo},
    futures::TryFutureExt,
};
use std::{env, path::PathBuf, process::exit};
use tokio::{fs::create_dir_all, sync::OnceCell};

pub async fn config_path(guild: &GuildId) -> Result<PathBuf, ModuleError> {
    debug!("fetching data path");
    let path = data_path().await?.join("config").join(guild.to_string());
    debug!("checking path exists");
    if !path.exists() {
        create_dir_all(&path).map_err(ConfigError::IoError).await?;
    }
    Ok(path)
}

pub async fn data_path() -> Result<PathBuf, ModuleError> {
    debug!("data_path");
    if !DATA_PATH.initialized() {
        DATA_PATH
            .set(if let Ok(data_path) = env::var("DATA_PATH") {
                data_path.into()
            } else {
                warn!("DATA_PATH not set, defaulting to local data directory.");

                dirs::data_dir()
                    .or_else(|| {
                        error!("failed to locate data directory");
                        exit(1);
                    })
                    .unwrap()
                    .join("dragon-bot")
            })
            .unwrap_or_else(|e| {
                error!("failed to set data path static cell: {e}");
                exit(1);
            });
        info!("Data Directory: {:?}", DATA_PATH.get().unwrap());

        let data_path = DATA_PATH.get().unwrap();
        if !data_path.exists() {
            debug!("creating base path: {}", data_path.to_string_lossy());
            create_dir_all(data_path)
                .await
                .map_err(ConfigError::IoError)?;
        }
    }

    Ok(DATA_PATH.get().unwrap().into())
}
static DATA_PATH: OnceCell<PathBuf> = OnceCell::const_new();

pub async fn get_all_guilds(ctx: &Context) -> Result<Vec<GuildInfo>, ModuleError> {
    let mut guilds: Vec<GuildInfo> = vec![];
    loop {
        debug!("{guilds:#?}");
        let mut fetched = ctx
            .http()
            .get_guilds(
                if guilds.is_empty() {
                    None
                } else {
                    Some(serenity::all::GuildPagination::After(
                        guilds.last().unwrap().id,
                    ))
                },
                Some(100),
            )
            .await
            .map_err(CommandError::Serenity)?;

        guilds.append(&mut fetched);
        if fetched.len() < 100 {
            break;
        }
    }

    Ok(guilds)
}

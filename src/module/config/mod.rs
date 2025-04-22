use super::errors::ModuleError;
use crate::{core::module::DragonBotModule, util::config_path};
use serde::{Deserialize, Serialize};
use serenity::{all::GuildId, async_trait};
use std::io;
use tokio::fs::read_to_string;

mod command;
mod permission;

#[derive(Debug)]
pub enum ConfigError {
    SerdeError(serde_json::Error),
    IoError(io::Error),
}

#[async_trait]
pub trait DragonModuleConfigurable: DragonBotModule {
    type Config: Serialize + for<'de> Deserialize<'de> + Default + Send;

    async fn get_config<T: DragonBotModule>(
        &self,
        guild: GuildId,
    ) -> Result<Self::Config, ModuleError> {
        let config_path = config_path(&guild)
            .await?
            .join(format!("{}.json", Self::module_id()));
        if !config_path.exists() {
            return Ok(Self::Config::default());
        }

        let json = read_to_string(config_path)
            .await
            .map_err(ConfigError::IoError)?;
        Ok(serde_json::from_str(&json).map_err(ConfigError::SerdeError)?)
    }

    async fn set_config<T: DragonBotModule>(
        &self,
        guild: GuildId,
        config: Self::Config,
    ) -> Result<(), ModuleError> {
        let config_path = config_path(&guild)
            .await?
            .join(format!("{}.json", Self::module_id()));
        let json = serde_json::to_string(&config)
            .map_err(ConfigError::SerdeError)?
            .to_string();

        tokio::fs::write(config_path, json)
            .await
            .map_err(ConfigError::IoError)?;

        Ok(())
    }
}

#[derive(Default)]
pub struct ConfigManager;

impl DragonBotModule for ConfigManager {
    fn module_id() -> &'static str
    where
        Self: Sized,
    {
        "config-manager"
    }
}

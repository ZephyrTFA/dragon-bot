use std::io;
use tokio::fs::read_to_string;

use serde::{Deserialize, Serialize};
use serenity::{all::GuildId, async_trait};

use crate::data_path;

use super::{DragonBotModule, errors::ModuleError};

#[derive(Debug)]
pub enum ConfigError {
    SerdeError(serde_json::Error),
    IoError(io::Error),
}

#[async_trait]
pub trait Configurable<C>
where
    Self: DragonBotModule + Send,
    C: Serialize + for<'de> Deserialize<'de> + Default + Send + 'static,
{
    async fn get_config(&self, guild: GuildId) -> Result<C, ModuleError> {
        let config_path = data_path()
            .join("config")
            .join(guild.to_string())
            .join(format!("{}.json", Self::module_id()));
        if !config_path.exists() {
            return Ok(C::default());
        }

        let json = read_to_string(config_path)
            .await
            .map_err(ConfigError::IoError)?;
        Ok(serde_json::from_str(&json).map_err(ConfigError::SerdeError)?)
    }

    async fn set_config(&self, guild: GuildId, config: C) -> Result<(), ModuleError> {
        let config_path = data_path()
            .join("config")
            .join(guild.to_string())
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
pub struct ConfigManager {}

impl DragonBotModule for ConfigManager {
    fn module_id() -> &'static str
    where
        Self: Sized,
    {
        "config-manager"
    }
}

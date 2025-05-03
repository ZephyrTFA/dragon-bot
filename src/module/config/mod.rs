use super::errors::ModuleError;
use crate::{core::module::DragonBotModule, util::config_path};
use entry::{ConfigField, ConfigFieldError, ConfigValue};
use serde::{Deserialize, Serialize};
use serenity::{all::GuildId, async_trait};
use std::{collections::HashMap, io};
use tokio::fs::read_to_string;

mod command;
pub mod entry;
mod permission;

#[derive(Debug)]
pub enum ConfigError {
    SerdeError(serde_json::Error),
    IoError(io::Error),
}

pub trait ModuleConfig: Serialize + for<'de> Deserialize<'de> + Default + Send {
    fn get_config_fields() -> HashMap<&'static str, ConfigField>;
    fn get_config_entry(&self, field: &str) -> Result<ConfigValue, ConfigFieldError>;
    fn set_config_entry(&mut self, field: &str, value: ConfigValue)
    -> Result<(), ConfigFieldError>;
}

#[derive(Serialize, Deserialize, Default)]
pub struct NoConfig;
impl ModuleConfig for NoConfig {
    fn get_config_entry(&self, _field: &str) -> Result<ConfigValue, ConfigFieldError> {
        Err(ConfigFieldError::FieldNotFound)
    }
    fn get_config_fields() -> HashMap<&'static str, ConfigField> {
        HashMap::new()
    }
    fn set_config_entry(
        &mut self,
        _field: &str,
        _value: ConfigValue,
    ) -> Result<(), ConfigFieldError> {
        Err(ConfigFieldError::FieldNotFound)
    }
}

#[async_trait]
pub trait DragonModuleConfigurable {
    type Config: ModuleConfig;
    type Module: DragonBotModule;

    fn get_config_fields() -> HashMap<&'static str, ConfigField> {
        Self::Config::get_config_fields()
    }

    async fn get_full_config(&self, guild: GuildId) -> Result<Self::Config, ModuleError> {
        let config_path = config_path(&guild)
            .await?
            .join(format!("{}.json", Self::Module::module_id()));
        if !config_path.exists() {
            return Ok(Self::Config::default());
        }

        let json = read_to_string(config_path)
            .await
            .map_err(ConfigError::IoError)?;
        Ok(serde_json::from_str(&json).map_err(ConfigError::SerdeError)?)
    }

    async fn set_full_config(
        &self,
        guild: GuildId,
        config: Self::Config,
    ) -> Result<(), ModuleError> {
        let config_path = config_path(&guild)
            .await?
            .join(format!("{}.json", Self::Module::module_id()));
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

impl DragonModuleConfigurable for ConfigManager {
    type Config = NoConfig;
    type Module = ConfigManager;
}

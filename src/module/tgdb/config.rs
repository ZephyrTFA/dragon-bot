use std::collections::HashMap;

use crate::module::config::{
    DragonModuleConfigurable, ModuleConfig,
    entry::{ConfigEntryType, ConfigField, ConfigFieldError, ConfigValue},
};

use super::TgDb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TgDbConfig {
    pub address: String,
    pub port: u64,
    pub user: String,
    pub password_b64: String,
    pub database: String,
}

impl ModuleConfig for TgDbConfig {
    fn get_config_fields() -> HashMap<&'static str, ConfigField> {
        [
            (
                "address",
                ConfigField::new(ConfigEntryType::String, "address to the database"),
            ),
            (
                "user",
                ConfigField::new(ConfigEntryType::String, "username for the client"),
            ),
            (
                "password_b64",
                ConfigField::new(
                    ConfigEntryType::String,
                    "base64 hash of the password for the client",
                ),
            ),
            (
                "database",
                ConfigField::new(ConfigEntryType::String, "name of the database to use"),
            ),
            (
                "port",
                ConfigField::new(ConfigEntryType::U64, "port to use for the database"),
            ),
        ]
        .into_iter()
        .collect()
    }

    fn get_config_entry(&self, field: &str) -> Result<ConfigValue, ConfigFieldError> {
        match field {
            "address" => Ok(ConfigValue::String(self.address.clone())),
            "port" => Ok(ConfigValue::U64(self.port)),
            "user" => Ok(ConfigValue::String(self.user.clone())),
            "password_b64" => Ok(ConfigValue::String(self.password_b64.clone())),
            "database" => Ok(ConfigValue::String(self.database.clone())),
            _ => Err(ConfigFieldError::FieldNotFound),
        }
    }

    fn set_config_entry(
        &mut self,
        field: &str,
        value: ConfigValue,
    ) -> Result<(), ConfigFieldError> {
        match field {
            "address" => self.address = value.to_string()?,
            "port" => self.port = value.to_u64()?,
            "user" => self.user = value.to_string()?,
            "password_b64" => self.password_b64 = value.to_string()?,
            "database" => self.database = value.to_string()?,
            _ => return Err(ConfigFieldError::FieldNotFound),
        }
        Ok(())
    }
}

impl DragonModuleConfigurable for TgDb {
    type Config = TgDbConfig;
    type Module = TgDb;
}

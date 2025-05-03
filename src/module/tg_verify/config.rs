use super::TgVerify;
use crate::module::config::{
    DragonModuleConfigurable, ModuleConfig,
    entry::{ConfigEntryType, ConfigField, ConfigFieldError, ConfigValue},
};
use serde::{Deserialize, Serialize};
use serenity::all::RoleId;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct TgVerifyConfig {
    pub role_verified_linked: RoleId,
    pub role_verified_living: RoleId,
    pub living_minutes_required: u64,
    pub table_playtime: String,
    pub table_linking: String,
}

impl ModuleConfig for TgVerifyConfig {
    fn get_config_fields() -> HashMap<&'static str, ConfigField> {
        [
            (
                "role_verified_linked",
                ConfigField::new(
                    ConfigEntryType::U64,
                    "role to give users for linking their BYOND account",
                ),
            ),
            (
                "role_verified_living",
                ConfigField::new(
                    ConfigEntryType::U64,
                    "role to give users who meet the minimum playtime threshold",
                ),
            ),
            (
                "living_minutes_required",
                ConfigField::new(
                    ConfigEntryType::U64,
                    "the playtime threshold for the verified living role",
                ),
            ),
            (
                "table_linking",
                ConfigField::new(
                    ConfigEntryType::String,
                    "the table to query for discord links",
                ),
            ),
            (
                "table_playtime",
                ConfigField::new(
                    ConfigEntryType::String,
                    "the table to query for player playtime",
                ),
            ),
        ]
        .into_iter()
        .collect()
    }

    fn set_config_entry(
        &mut self,
        field: &str,
        value: ConfigValue,
    ) -> Result<(), ConfigFieldError> {
        match field {
            "role_verified_linked" => self.role_verified_linked = value.to_u64()?.into(),
            "role_verified_living" => self.role_verified_living = value.to_u64()?.into(),
            "living_minutes_required" => self.living_minutes_required = value.to_u64()?,
            "table_linking" => self.table_linking = value.to_string()?,
            "table_playtime" => self.table_playtime = value.to_string()?,
            _ => return Err(ConfigFieldError::FieldNotFound),
        }
        Ok(())
    }

    fn get_config_entry(&self, field: &str) -> Result<ConfigValue, ConfigFieldError> {
        match field {
            "role_verified_linked" => Ok(ConfigValue::U64(self.role_verified_linked.get())),
            "role_verified_living" => Ok(ConfigValue::U64(self.role_verified_living.get())),
            "living_minutes_required" => Ok(ConfigValue::U64(self.living_minutes_required)),
            "table_linking" => Ok(ConfigValue::String(self.table_linking.clone())),
            "table_playtime" => Ok(ConfigValue::String(self.table_playtime.clone())),
            _ => Err(ConfigFieldError::FieldNotFound),
        }
    }
}

impl DragonModuleConfigurable for TgVerify {
    type Config = TgVerifyConfig;
    type Module = TgVerify;
}

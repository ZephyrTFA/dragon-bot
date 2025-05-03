use super::ModuleManager;
use crate::module::config::{
    DragonModuleConfigurable, ModuleConfig,
    entry::{ConfigField, ConfigFieldError, ConfigValue},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct ModuleManagerConfig {
    pub active: Vec<String>,
}

impl ModuleConfig for ModuleManagerConfig {
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

    fn get_config_entry(&self, _field: &str) -> Result<ConfigValue, ConfigFieldError> {
        Err(ConfigFieldError::FieldNotFound)
    }
}

impl DragonModuleConfigurable for ModuleManager {
    type Config = ModuleManagerConfig;
    type Module = ModuleManager;
}

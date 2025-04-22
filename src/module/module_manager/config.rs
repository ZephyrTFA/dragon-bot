use serde::{Deserialize, Serialize};

use crate::module::config::DragonModuleConfigurable;

use super::ModuleManager;

#[derive(Serialize, Deserialize, Default)]
pub struct ModuleManagerConfig {
    pub active: Vec<String>,
}

impl DragonModuleConfigurable for ModuleManager {
    type Config = ModuleManagerConfig;
}

use std::collections::HashMap;

use super::{
    DragonBotModule, config::ConfigError, manager::ModuleManagerError,
    permissions::PermissionsError, tgdb::TgDbError,
};
use log::error;

#[derive(Debug)]
pub enum ModuleError {
    TgDbError(TgDbError),
    ModuleManagerError(ModuleManagerError),
    PermissionsError(PermissionsError),
    ConfigError(ConfigError),
}

macro_rules! impl_from {
    ($type: ident) => {
        impl From<$type> for ModuleError {
            fn from(value: $type) -> Self {
                Self::$type(value)
            }
        }
    };
}

impl_from!(TgDbError);
impl_from!(ModuleManagerError);
impl_from!(PermissionsError);
impl_from!(ConfigError);

#[derive(Default)]
pub struct ErrorManager {
    all_error_log: Vec<String>,
    module_error_log: HashMap<String, Vec<String>>,
}

impl DragonBotModule for ErrorManager {
    fn module_id() -> &'static str
    where
        Self: Sized,
    {
        "error-manager"
    }
}

impl ErrorManager {
    pub fn module_error(&mut self, module: &impl DragonBotModule, error: ModuleError) {
        let error_string = match &error {
            ModuleError::TgDbError(e) => {
                format!("[{}] TgDbError: {:?}", module.id(), e)
            }
            ModuleError::ModuleManagerError(e) => {
                format!("[{}] ModuleManagerError: {:?}", module.id(), e)
            }
            ModuleError::PermissionsError(e) => {
                format!("[{}] PermissionsError: {:?}", module.id(), e)
            }
            ModuleError::ConfigError(e) => {
                format!("[{}] ConfigError: {:?}", module.id(), e)
            }
        };
        error!("{}", &error_string);
        self.all_error_log.push(error_string.clone());
        self.module_error_log
            .entry(module.id().to_string())
            .or_default()
            .push(error_string);
    }
}

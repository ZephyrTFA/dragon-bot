use super::{
    super::core::module::GetModuleError,
    commands::CommandError,
    config::{ConfigError, DragonModuleConfigurable, NoConfig},
    module_manager::ModuleManagerError,
    permissions::PermissionsError,
    tgdb::TgDbError,
};
use crate::core::module::DragonBotModule;
use log::error;
use std::collections::HashMap;

mod command;
mod permissions;

macro_rules! module_error_types {
    ( $( $type: ident ),+ ) => {
        #[derive(Debug)]
        pub enum ModuleError {
            $(
                $type($type),
            )+
        }

        $(
            impl From<$type> for ModuleError {
                fn from(value: $type) -> Self {
                    Self::$type(value)
                }
            }
        )+

        impl ErrorManager {
            fn get_module_error_string(module: &impl DragonBotModule, error: &ModuleError) -> String where {
                match error {
                    $(
                        ModuleError::$type(err) => format!("[{}][{}]: {:?}", module.id(), stringify!($type), err),
                    )+
                }
            }
        }
    };
}

module_error_types! {
    TgDbError,
    ModuleManagerError,
    PermissionsError,
    ConfigError,
    CommandError,
    GetModuleError
}

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

impl DragonModuleConfigurable for ErrorManager {
    type Config = NoConfig;
    type Module = ErrorManager;
}

impl ErrorManager {
    pub fn module_error(&mut self, module: &impl DragonBotModule, error: &ModuleError) {
        let error_string = ErrorManager::get_module_error_string(module, error);

        error!("{}", &error_string);
        self.all_error_log.push(error_string.clone());
        self.module_error_log
            .entry(module.id().to_string())
            .or_default()
            .push(error_string);
    }
}

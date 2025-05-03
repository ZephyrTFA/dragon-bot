use crate::core::commands::DragonModuleCommand;
use crate::core::module::DragonBotModule;
use crate::core::permissions::DragonModulePermission;
use crate::core::permissions::ModulePermission;
use crate::module::config::DragonModuleConfigurable;
use crate::module::config::ModuleConfig;
use crate::module::config::NoConfig;
use crate::module::config::entry::ConfigField;
use crate::module::config::entry::ConfigFieldError;
use crate::module::config::entry::ConfigValue;
use crate::module::errors::ModuleError;
use crate::module::module_manager::ModuleManager;
use crate::module::module_manager::config::ModuleManagerConfig;
use crate::module::permissions::config::PermissionsManagerConfig;
use crate::module::tg_verify::config::TgVerifyConfig;
use crate::module::tgdb::config::TgDbConfig;
use crate::module::{
    config::ConfigManager, errors::ErrorManager, permissions::PermissionsManager,
    tg_verify::TgVerify, tgdb::TgDb,
};
use serenity::all::{CommandInteraction, Context, CreateCommand, GuildId};
use std::collections::HashMap;
use strum::EnumIter;

#[derive(EnumIter)]
pub enum DragonBotModuleInstance {
    TgDb(TgDb),
    TgVerify(TgVerify),
    ConfigManager(ConfigManager),
    PermissionsManager(PermissionsManager),
    ErrorManager(ErrorManager),
    ModuleManager(ModuleManager),
}

pub enum ModuleConfigHolder {
    TgDb(TgDbConfig),
    TgVerify(TgVerifyConfig),
    ConfigManager(NoConfig),
    PermissionsManager(PermissionsManagerConfig),
    ErrorManager(NoConfig),
    ModuleManager(ModuleManagerConfig),
}

impl_from!(
    TgDb
    TgVerify
    ConfigManager
    PermissionsManager
    ErrorManager
    ModuleManager
);

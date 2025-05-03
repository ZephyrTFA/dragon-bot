use crate::core::commands::DragonModuleCommand;
use crate::core::module::DragonBotModule;
use crate::core::permissions::DragonModulePermission;
use crate::core::permissions::ModulePermission;
use crate::module::config::DragonModuleConfigurable;
use crate::module::config::entry::ConfigField;
use crate::module::errors::ModuleError;
use crate::module::module_manager::ModuleManager;
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

impl_from!(
    TgDb,
    TgVerify,
    ConfigManager,
    PermissionsManager,
    ErrorManager,
    ModuleManager
);

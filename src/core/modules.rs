use crate::core::commands::DragonModuleCommand;
use crate::core::module::DragonBotModule;
use crate::module::errors::ModuleError;
use crate::module::module_manager::{ModuleManager, ModuleManagerError};
use crate::module::{
    config::ConfigManager, errors::ErrorManager, permissions::PermissionsManager,
    tg_verify::TgVerify, tgdb::TgDb,
};
use log::debug;
use serenity::all::{CommandInteraction, Context, CreateCommand};
use std::collections::HashMap;
use tokio::sync::{Mutex, MutexGuard, OnceCell};

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

static MODULES: OnceCell<HashMap<String, Mutex<DragonBotModuleInstance>>> = OnceCell::const_new();

async fn setup_modules() -> HashMap<String, Mutex<DragonBotModuleInstance>> {
    init_modules()
        .into_iter()
        .map(|(module, instance)| (module, Mutex::new(instance)))
        .collect()
}

pub async fn get_module_instance<'a, M>() -> MutexGuard<'a, DragonBotModuleInstance>
where
    M: DragonBotModule + 'a,
    &'a M: From<&'a DragonBotModuleInstance>,
{
    get_module_instance_by_id(M::module_id()).await.unwrap()
}

pub async fn get_module_instance_by_id<'a>(
    id: &str,
) -> Result<MutexGuard<'a, DragonBotModuleInstance>, ModuleError> {
    debug!("get_module_instance_by_id: {id}");
    let modules = MODULES.get_or_init(setup_modules).await;
    Ok(modules
        .get(id)
        .ok_or(ModuleManagerError::ModuleNotFound)?
        .lock()
        .await)
}

#[macro_export]
macro_rules! get_module {
    ($var: ident, $instance: ident, $module: ident) => {
        let $instance = $crate::core::modules::get_module_instance::<$module>().await;
        let $var: &$module = tokio::sync::MutexGuard::deref(&$instance).into();
    };
}

#[macro_export]
macro_rules! get_module_mut {
    ($var: ident, $module: ident) => {
        let mut $var = $crate::core::modules::get_module_instance::<$module>().await;
        let $var: &mut $module = tokio::sync::MutexGuard::deref_mut(&mut $var).into();
    };
}

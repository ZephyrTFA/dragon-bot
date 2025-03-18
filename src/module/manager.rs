use std::collections::HashMap;

use tokio::sync::{Mutex, MutexGuard, OnceCell};

use super::{DragonBotModule, errors::ModuleError, modules::DragonBotModuleInstance};

#[derive(Debug)]
pub enum ModuleManagerError {
    ModuleNotLoaded,
}

#[derive(Default)]
pub struct ModuleManager {
    modules: HashMap<String, DragonBotModuleInstance>,
}

impl DragonBotModule for ModuleManager {
    fn module_id() -> &'static str
    where
        Self: Sized,
    {
        "module-manager"
    }
}

static MODULE_MANAGER: OnceCell<Mutex<ModuleManager>> = OnceCell::const_new();

pub async fn module_manager<'a>() -> MutexGuard<'a, ModuleManager> {
    MODULE_MANAGER
        .get_or_init(async || Mutex::new(ModuleManager::default()))
        .await
        .lock()
        .await
}

impl<'a> ModuleManager {
    pub fn get_module<M>(&'a self) -> Result<&'a M, ModuleError>
    where
        M: DragonBotModule,
        &'a M: From<&'a DragonBotModuleInstance>,
    {
        Ok(self
            .modules
            .get(M::module_id())
            .map(|instance| instance.into())
            .ok_or(ModuleManagerError::ModuleNotLoaded)?)
    }

    pub fn get_module_mut<M>(&'a mut self) -> Result<&'a mut M, ModuleError>
    where
        M: DragonBotModule + Send,
        &'a mut M: From<&'a mut DragonBotModuleInstance>,
    {
        Ok(self
            .modules
            .get_mut(M::module_id())
            .map(|instance| instance.into())
            .ok_or(ModuleManagerError::ModuleNotLoaded)?)
    }

    pub fn load_module<M>(&'a mut self) -> Result<(), ModuleManagerError>
    where
        M: DragonBotModule,
    {
        todo!()
    }
}

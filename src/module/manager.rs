use std::{collections::HashMap, fs::exists};

use serenity::all::{CacheHttp, Context, GuildId};
use tokio::{
    fs::read_to_string,
    sync::{Mutex, MutexGuard, OnceCell},
};

use crate::config_path;

use super::{
    DragonBotModule, config::ConfigError, errors::ModuleError, modules::DragonBotModuleInstance,
};

#[derive(Debug)]
pub enum ModuleManagerError {
    ModuleNotActive,
}

#[derive(Default)]
pub struct ModuleManager {
    modules: HashMap<String, DragonBotModuleInstance>,
    active_modules: HashMap<GuildId, Vec<String>>,
}

impl DragonBotModule for ModuleManager {
    fn module_id() -> &'static str
    where
        Self: Sized,
    {
        "module-manager"
    }

    async fn init(&mut self, ctx: &Context) -> Result<(), ModuleError> {
        for guild in ctx.http().get_guilds(None, None).await.iter().flatten() {
            let loaded_modules_path = config_path(&guild.id)
                .await
                .map_err(ConfigError::IoError)?
                .join("modules.json");
            if exists(&loaded_modules_path).is_err() {
                continue;
            }

            let target_loaded: Vec<String> = serde_json::from_str(
                read_to_string(&loaded_modules_path)
                    .await
                    .map_err(ConfigError::IoError)?
                    .as_str(),
            )
            .map_err(ConfigError::SerdeError)?;

            let mut module_manager = module_manager().await;
            for target in target_loaded {
                module_manager.load_module(&target).await?;
            }
        }

        Ok(())
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
    pub fn get_module_by_id(
        &'a self,
        guild: GuildId,
        id: &str,
    ) -> Result<&'a DragonBotModuleInstance, ModuleError> {
        if !self
            .active_modules
            .get(&guild)
            .is_some_and(|am| am.contains(&id.to_string()))
        {
            Err(ModuleManagerError::ModuleNotActive)?;
            unreachable!()
        }

        Ok(self.modules.get(id).unwrap())
    }

    pub fn get_module<M>(&'a self, guild: GuildId) -> Result<&'a M, ModuleError>
    where
        M: DragonBotModule,
        &'a M: From<&'a DragonBotModuleInstance>,
    {
        self.get_module_by_id(guild, M::module_id())
            .map(|module| module.into())
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
            .ok_or(ModuleManagerError::ModuleNotActive)?)
    }

    pub fn get_active_modules(&self, guild: GuildId) -> Vec<&DragonBotModuleInstance> {
        let active_modules = self.active_modules.get(&guild);
        if active_modules.is_none() {
            return vec![];
        }

        self.active_modules
            .get(&guild)
            .unwrap()
            .iter()
            .map(|mid| self.get_module_by_id(guild, mid).unwrap())
            .collect()
    }

    pub async fn load_module(&'a mut self, _module: &str) -> Result<(), ModuleManagerError> {
        todo!()
    }
}

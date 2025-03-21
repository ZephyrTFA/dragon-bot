use std::{collections::HashMap, path::PathBuf};

use super::{config::ConfigError, errors::ModuleError};
use crate::{
    core::{module::DragonBotModule, modules::get_module_instance_by_id},
    data_path,
};
use log::{debug, info, warn};
use serenity::all::{CacheHttp, Context, GuildId};
use tokio::fs::{create_dir_all, read_to_string, write};

mod command;
mod permission;

#[derive(Debug)]
pub enum ModuleManagerError {
    ModuleNotActive,
    ModuleAlreadyActive,
    ModuleAlreadyInactive,
    CannotInactivateManager,
    LoadActiveFailed,
    ModuleNotFound,
}

#[derive(Default)]
pub struct ModuleManager {
    active_modules: HashMap<GuildId, Vec<String>>,
}

async fn active_module_store_path(guild: GuildId) -> Result<PathBuf, ModuleError> {
    let base = data_path().await?.join("active");
    if !base.exists() {
        create_dir_all(&base).await.map_err(ConfigError::IoError)?;
    }
    Ok(base.join(format!("{guild}.json")))
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
            info!("Loading active modules for {}|({})", guild.id, guild.name);
            self.load_active(guild.id).await?;
        }

        Ok(())
    }
}

impl<'a> ModuleManager {
    pub fn get_all_active_module_ids(&self, guild: GuildId) -> Vec<String> {
        let mut active_modules = self.active_modules.get(&guild).cloned().unwrap_or_default();
        active_modules.push(Self::module_id().to_string());
        active_modules.sort();
        active_modules
    }

    pub fn is_module_active<M>(&self, guild: GuildId) -> bool
    where
        M: DragonBotModule,
    {
        self.is_module_id_active(guild, M::module_id())
    }

    pub fn is_module_id_active(&self, guild: GuildId, module: &str) -> bool {
        if module == Self::module_id() {
            return true;
        }

        self.active_modules
            .get(&guild)
            .is_some_and(|active| active.contains(&module.to_string()))
    }

    async fn save_active(&self, guild: GuildId) -> Result<(), ModuleError> {
        let json = serde_json::to_string_pretty(self.active_modules.get(&guild).unwrap())
            .map_err(ConfigError::SerdeError)?;
        let store = active_module_store_path(guild).await?;
        write(store, json).await.map_err(ConfigError::IoError)?;
        Ok(())
    }

    async fn load_active(&mut self, guild: GuildId) -> Result<(), ModuleError> {
        debug!("loading active for {guild}");

        let store = active_module_store_path(guild).await?;
        if !store.exists() {
            self.active_modules
                .insert(guild, vec![self.id().to_string()]);
            return Ok(());
        }

        let json = read_to_string(store).await.map_err(ConfigError::IoError)?;
        let mut wanted_active =
            serde_json::from_str::<Vec<String>>(&json).map_err(ConfigError::SerdeError)?;

        // always ensure that we are NOT in the active list
        wanted_active.retain(|wanted| wanted != Self::module_id());

        let mut will_active = vec![];
        for wanted in wanted_active {
            let instance = get_module_instance_by_id(&wanted).await;
            if instance.is_ok() {
                will_active.push(wanted);
            } else {
                warn!("Removing wanted active invalid module: {wanted}");
            }
            drop(instance);
        }

        self.active_modules.insert(guild, will_active);
        Ok(())
    }

    pub async fn set_module_active(
        &'a mut self,
        guild: GuildId,
        module: &str,
    ) -> Result<(), ModuleError> {
        info!("Setting module {module} active for {guild}");

        if self.is_module_id_active(guild, module) {
            Err(ModuleManagerError::ModuleAlreadyActive)?;
            unreachable!();
        }

        _ = get_module_instance_by_id(module).await?;
        self.active_modules.entry(guild).or_default();
        self.active_modules
            .get_mut(&guild)
            .unwrap()
            .push(module.to_string());
        self.save_active(guild).await?;

        Ok(())
    }

    pub async fn set_module_inactive(
        &'a mut self,
        guild: GuildId,
        module: &str,
    ) -> Result<(), ModuleError> {
        if module.eq(self.id()) {
            Err(ModuleManagerError::CannotInactivateManager)?;
            unreachable!();
        }

        _ = get_module_instance_by_id(module).await?;
        if !self.is_module_id_active(guild, module) {
            Err(ModuleManagerError::ModuleAlreadyInactive)?;
            unreachable!();
        }

        self.active_modules
            .get_mut(&guild)
            .unwrap()
            .retain(|m| m != module);
        self.save_active(guild).await?;

        Ok(())
    }
}

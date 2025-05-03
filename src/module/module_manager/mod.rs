use super::{config::DragonModuleConfigurable, errors::ModuleError};
use crate::core::{module::DragonBotModule, modules::DragonBotModuleInstance};
use log::info;
use serenity::all::GuildId;

mod command;
pub mod config;
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

#[derive(Default, Debug)]
pub struct ModuleManager {}

impl DragonBotModule for ModuleManager {
    fn module_id() -> &'static str
    where
        Self: Sized,
    {
        "module-manager"
    }
}

impl<'a> ModuleManager {
    pub async fn get_all_active_module_ids(
        &self,
        guild: GuildId,
    ) -> Result<Vec<String>, ModuleError> {
        let config = Self::get_full_config(guild).await?;
        Ok(config.active.clone())
    }

    pub async fn is_module_active<M>(&self, guild: GuildId) -> Result<bool, ModuleError>
    where
        M: DragonBotModule,
    {
        self.is_module_id_active(guild, M::module_id()).await
    }

    pub async fn is_module_id_active(
        &self,
        guild: GuildId,
        module: &str,
    ) -> Result<bool, ModuleError> {
        if module == Self::module_id() {
            return Ok(true);
        }

        Ok(self
            .get_all_active_module_ids(guild)
            .await?
            .contains(&module.to_string()))
    }

    pub async fn set_module_active(
        &'a mut self,
        guild: GuildId,
        module: &str,
    ) -> Result<(), ModuleError> {
        info!("Setting module {module} active for {guild}");

        if self.is_module_id_active(guild, module).await? {
            Err(ModuleManagerError::ModuleAlreadyActive)?;
            unreachable!();
        }

        if !DragonBotModuleInstance::all_module_ids().contains(&module) {
            Err(ModuleManagerError::ModuleNotFound)?;
            unreachable!();
        }

        let mut config = Self::get_full_config(guild).await?;
        config.active.push(module.to_string());
        Self::set_full_config(guild, config).await?;

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

        if !DragonBotModuleInstance::all_module_ids().contains(&module) {
            Err(ModuleManagerError::ModuleNotFound)?;
            unreachable!();
        }

        if !self.is_module_id_active(guild, module).await? {
            Err(ModuleManagerError::ModuleAlreadyInactive)?;
            unreachable!();
        }

        let mut config = Self::get_full_config(guild).await?;
        config.active.retain(|m| m != module);
        Self::set_full_config(guild, config).await?;

        Ok(())
    }
}

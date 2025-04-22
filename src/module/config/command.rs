use std::time::Duration;

use super::ConfigManager;
use crate::{
    core::{
        commands::DragonModuleCommand,
        module::{DragonBotModule, get_module_by_id},
        modules::DragonBotModuleInstance,
    },
    module::errors::ModuleError,
};
use log::warn;
use serenity::all::{CommandInteraction, Context, CreateCommand};

impl DragonModuleCommand for ConfigManager {
    async fn command_builder(&self) -> Option<CreateCommand> {
        let mut _toplevel = CreateCommand::new(ConfigManager::module_id());

        for module in DragonBotModuleInstance::all_module_ids()
            .iter()
            .map(|id| get_module_by_id(id, Some(Duration::from_secs(5))))
        {
            let _module = module.await.unwrap();
        }

        warn!("todo: ConfigManager::command_builder");
        None
    }

    async fn command_handle(
        &mut self,
        _ctx: &Context,
        _interaction: &CommandInteraction,
    ) -> Result<(), ModuleError> {
        todo!()
    }
}

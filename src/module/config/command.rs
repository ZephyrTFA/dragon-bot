use super::ConfigManager;
use crate::{core::commands::DragonModuleCommand, module::errors::ModuleError};
use log::warn;
use serenity::all::{CommandInteraction, Context, CreateCommand};

impl DragonModuleCommand for ConfigManager {
    fn command_builder(&self) -> Option<CreateCommand> {
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

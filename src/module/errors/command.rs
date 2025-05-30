use log::warn;
use serenity::all::{CommandInteraction, Context, CreateCommand, GuildId};

use crate::core::commands::DragonModuleCommand;

use super::{ErrorManager, ModuleError};

impl DragonModuleCommand for ErrorManager {
    async fn command_builder(&self, _guild: GuildId) -> Option<CreateCommand> {
        warn!("todo: ErrorManager::command_builder");
        None
    }

    async fn command_handle(
        &mut self,
        _ctx: &Context,
        _command: &CommandInteraction,
    ) -> Result<(), ModuleError> {
        todo!()
    }
}

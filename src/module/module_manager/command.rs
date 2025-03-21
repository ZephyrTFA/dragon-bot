use std::ops::Deref;

use super::ModuleManager;
use crate::{
    core::{
        commands::DragonModuleCommand, event_handler::ModuleEventHandler, module::DragonBotModule,
        modules::get_module_instance_by_id,
    },
    module::errors::ModuleError,
};
use log::warn;
use serenity::all::{
    CacheHttp, CommandDataOptionValue, CommandInteraction, CommandOptionType, Context,
    CreateCommand, CreateCommandOption, CreateInteractionResponseFollowup,
};

impl DragonModuleCommand for ModuleManager {
    fn command_builder(&self) -> Option<CreateCommand> {
        Some(
            CreateCommand::new(ModuleManager::module_id())
                .description("module manager")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::SubCommand,
                        "activate",
                        "activate a module",
                    )
                    .add_sub_option(
                        CreateCommandOption::new(
                            CommandOptionType::String,
                            "module",
                            "the module id",
                        )
                        .required(true),
                    ),
                )
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::SubCommand,
                        "deactivate",
                        "deactivate a module",
                    )
                    .add_sub_option(
                        CreateCommandOption::new(
                            CommandOptionType::String,
                            "module",
                            "the module id",
                        )
                        .required(true),
                    ),
                ),
        )
    }

    async fn command_handle(
        &mut self,
        ctx: &Context,
        command: &CommandInteraction,
    ) -> Result<(), ModuleError> {
        let subcommand = command.data.options.first();
        if subcommand.is_none() {
            self.command_help(ctx, command).await;
            return Ok(());
        }
        let subcommand = subcommand.unwrap();
        let guild = command.guild_id.expect("no guild id");

        match subcommand.name.as_str() {
            "activate" => match &subcommand.value {
                CommandDataOptionValue::SubCommand(target) => {
                    let target = target
                        .first()
                        .expect("required field not present")
                        .value
                        .as_str()
                        .expect("field malformed");
                    self.set_module_active(guild, target).await?;
                    if let Err(err) = command
                        .create_followup(
                            ctx.http(),
                            CreateInteractionResponseFollowup::new()
                                .content(format!("Module `{target}` activated.")),
                        )
                        .await
                    {
                        warn!("Failed to send interaction response: {err}");
                    }
                    let module = get_module_instance_by_id(target).await?;
                    ModuleEventHandler::register_guild_module_command(ctx, guild, module.deref())
                        .await;
                }
                _ => unreachable!(),
            },
            "deactivate" => match &subcommand.value {
                CommandDataOptionValue::SubCommand(target) => {
                    let target = target
                        .first()
                        .expect("required field not present")
                        .value
                        .as_str()
                        .expect("field malformed");
                    self.set_module_inactive(command.guild_id.unwrap(), target)
                        .await?;
                    if let Err(err) = command
                        .create_followup(
                            ctx.http(),
                            CreateInteractionResponseFollowup::new()
                                .content(format!("Module `{target}` deactivated.")),
                        )
                        .await
                    {
                        warn!("Failed to send interaction response: {err}");
                    }
                    let module = get_module_instance_by_id(target).await?;
                    ModuleEventHandler::drop_guild_module_command(ctx, guild, module.deref()).await;
                }
                _ => unreachable!(),
            },
            e => warn!("unknown module manager subcommand: {e}"),
        }

        Ok(())
    }
}

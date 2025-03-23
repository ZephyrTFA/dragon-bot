use super::{ModuleManager, permission::PERMISSION_MODULE_ACTIVATE};
use crate::{
    core::{
        commands::DragonModuleCommand,
        event_handler::ModuleEventHandler,
        module::{DragonBotModule, get_module_by_id},
        modules::DragonBotModuleInstance,
        permissions::assert_permission,
    },
    module::errors::ModuleError,
};
use log::warn;
use serenity::all::{
    CacheHttp, CommandDataOptionValue, CommandInteraction, CommandOptionType, Context,
    CreateCommand, CreateCommandOption, CreateInteractionResponseFollowup,
};

impl DragonModuleCommand for ModuleManager {
    async fn command_builder(&self) -> Option<CreateCommand> {
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
                )
                .add_option(CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "list-all",
                    "list all available modules",
                ))
                .add_option(CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "list-active",
                    "list all active modules",
                )),
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
            "activate"
                if assert_permission(
                    ctx,
                    command,
                    self,
                    command.member.as_ref().unwrap(),
                    PERMISSION_MODULE_ACTIVATE,
                )
                .await? =>
            {
                match &subcommand.value {
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
                        let module = get_module_by_id(target).await.unwrap();
                        ModuleEventHandler::register_guild_module_command(
                            ctx,
                            guild,
                            module.instance(),
                        )
                        .await;
                    }
                    _ => unreachable!(),
                }
            }
            "deactivate"
                if assert_permission(
                    ctx,
                    command,
                    self,
                    command.member.as_ref().unwrap(),
                    PERMISSION_MODULE_ACTIVATE,
                )
                .await? =>
            {
                match &subcommand.value {
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
                        let module = get_module_by_id(target).await.unwrap();
                        ModuleEventHandler::drop_guild_module_command(
                            ctx,
                            guild,
                            module.instance(),
                        )
                        .await;
                    }
                    _ => unreachable!(),
                }
            }
            "list-active" => {
                let mut response = "```diff\n".to_string();
                for active in self.get_all_active_module_ids(guild) {
                    response.push_str(format!("+ {}\n", active).as_str());
                }
                response.push_str("```\n");
                if let Err(error) = command
                    .create_followup(
                        ctx.http(),
                        CreateInteractionResponseFollowup::new().content(response),
                    )
                    .await
                {
                    warn!("Failed to send interaction response: {error}");
                };
            }
            "list-all" => {
                let mut response = "```diff\n".to_string();
                for module in DragonBotModuleInstance::all_module_ids() {
                    response.push_str(format!("{}\n", module).as_str());
                }
                response.push_str("```\n");
                if let Err(error) = command
                    .create_followup(
                        ctx.http(),
                        CreateInteractionResponseFollowup::new().content(response),
                    )
                    .await
                {
                    warn!("Failed to send interaction response: {error}");
                };
            }
            e => warn!("unknown module manager subcommand: {e}"),
        }

        Ok(())
    }
}

use super::PermissionsManager;
use crate::{
    core::{
        commands::DragonModuleCommand,
        module::{DragonBotModule, get_module, get_module_by_id},
        permissions::assert_permission,
    },
    module::{
        errors::ModuleError, module_manager::ModuleManager,
        permissions::permission::EDIT_PERMISSIONS,
    },
};
use log::warn;
use serenity::all::{
    CacheHttp, CommandDataOptionValue, CommandInteraction, CommandOptionType, Context,
    CreateAllowedMentions, CreateCommand, CreateCommandOption, CreateInteractionResponseFollowup,
    GuildId,
};

impl DragonModuleCommand for PermissionsManager {
    async fn command_builder(&self, guild: GuildId) -> Option<CreateCommand> {
        let mut builder = CreateCommand::new(self.id()).description("manage permissions");

        let module_manager =
            get_module::<ModuleManager>().expect("failed to get module manager for reading");
        let active = module_manager
            .module::<ModuleManager>()
            .get_all_active_module_ids(guild)
            .await
            .expect("failed to get active modules")
            .clone();

        for module_id in &active {
            let module = get_module_by_id(module_id).unwrap();
            let mut module_option =
                CreateCommandOption::new(CommandOptionType::SubCommandGroup, module_id, module_id);

            let mut grant_option = CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "grant",
                "grant a permission",
            );
            let mut revoke_option = CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "revoke",
                "revoke a permission",
            );

            let mut permission_option =
                CreateCommandOption::new(CommandOptionType::String, "permission", "the permission")
                    .required(true);
            for permission in module.all_permissions().await {
                permission_option =
                    permission_option.add_string_choice(permission.id(), permission.id());
            }

            let target_option =
                CreateCommandOption::new(CommandOptionType::Mentionable, "target", "target")
                    .required(true);

            grant_option = grant_option
                .add_sub_option(permission_option.clone())
                .add_sub_option(target_option.clone());
            revoke_option = revoke_option
                .add_sub_option(permission_option)
                .add_sub_option(target_option);

            module_option = module_option
                .add_sub_option(grant_option)
                .add_sub_option(revoke_option);
            builder = builder.add_option(module_option);
        }

        Some(builder)
    }

    async fn command_handle(
        &mut self,
        ctx: &Context,
        command: &CommandInteraction,
    ) -> Result<(), ModuleError> {
        let options = &command.data.options;

        if options.is_empty() {
            self.command_help(ctx, command).await;
            return Ok(());
        }

        let module = options.first().unwrap();
        if let CommandDataOptionValue::SubCommandGroup(operation) = &module.value {
            let operation = operation.first().unwrap();

            if let CommandDataOptionValue::SubCommand(data) = &operation.value {
                let target = &data[data.iter().position(|i| i.name == "target").unwrap()]
                    .value
                    .as_mentionable()
                    .unwrap();
                let permission = &data[data.iter().position(|i| i.name == "permission").unwrap()]
                    .value
                    .as_str()
                    .unwrap();

                let member = command.member.as_ref().unwrap();
                let guild = member.guild_id;
                match operation.name.as_str() {
                    "grant"
                        if assert_permission(ctx, command, member, EDIT_PERMISSIONS).await? =>
                    {
                        self.give_permission_str(guild, *target, &module.name, permission)
                            .await?;
                        if let Err(error) = command
                            .create_followup(
                                ctx.http(),
                                CreateInteractionResponseFollowup::new()
                                    .content(format!(
                                        "Granted `{}:{}` to {}",
                                        module.name, permission, target
                                    ))
                                    .allowed_mentions(CreateAllowedMentions::new()),
                            )
                            .await
                        {
                            warn!("failed to create followup: {error:?}");
                        }
                    }
                    "revoke"
                        if assert_permission(ctx, command, member, EDIT_PERMISSIONS).await? =>
                    {
                        self.take_permission_str(guild, *target, &module.name, permission)
                            .await?;
                        if let Err(error) = command
                            .create_followup(
                                ctx.http(),
                                CreateInteractionResponseFollowup::new()
                                    .content(format!(
                                        "Revoked `{}:{}` from {}",
                                        module.name, permission, target
                                    ))
                                    .allowed_mentions(CreateAllowedMentions::new()),
                            )
                            .await
                        {
                            warn!("failed to create followup: {error:?}");
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

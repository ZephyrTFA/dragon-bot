use super::{
    ConfigManager,
    entry::{ConfigEntryType, ConfigValue},
};
use crate::{
    core::{
        commands::DragonModuleCommand,
        module::{DragonBotModule, get_module, get_module_by_id},
    },
    module::{errors::ModuleError, module_manager::ModuleManager},
};
use core::panic;
use log::{debug, warn};
use serenity::all::{
    CacheHttp, ChannelType, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, EditInteractionResponse, GuildId, ResolvedValue,
};

impl DragonModuleCommand for ConfigManager {
    async fn command_builder(&self, guild: GuildId) -> Option<CreateCommand> {
        let mut toplevel = CreateCommand::new(ConfigManager::module_id())
            .description("manage user facing config settings for active modules.");

        let module_manager =
            get_module::<ModuleManager>().expect("failed to get module manager for reading");
        let module: &ModuleManager = module_manager.module();
        let active = module
            .get_all_active_module_ids(guild)
            .await
            .expect("failed to get active modules")
            .clone();

        for module in active
            .iter()
            .filter(|id| *id != "module-manager")
            .map(|id| get_module_by_id(id))
        {
            let module = module.unwrap();
            let mut module_subcommand = CreateCommandOption::new(
                CommandOptionType::SubCommandGroup,
                module.module_id(),
                format!("config for {}", module.module_id()),
            );

            for (field, field_data) in module.get_config_fields() {
                let mut field_option = CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    field,
                    field_data.description,
                );
                field_option = field_option.add_sub_option(match field_data.field_type {
                    ConfigEntryType::Role => {
                        CreateCommandOption::new(CommandOptionType::Role, "value", "role value")
                    }
                    ConfigEntryType::User => {
                        CreateCommandOption::new(CommandOptionType::User, "value", "user value")
                    }
                    ConfigEntryType::U64 => {
                        CreateCommandOption::new(CommandOptionType::Number, "value", "u64 value")
                    }
                    ConfigEntryType::ChannelText => CreateCommandOption::new(
                        CommandOptionType::Channel,
                        "value",
                        "channel value",
                    )
                    .channel_types(vec![ChannelType::Text]),
                    ConfigEntryType::String => {
                        CreateCommandOption::new(CommandOptionType::String, "value", "string value")
                    }
                });
                module_subcommand = module_subcommand.add_sub_option(field_option);
            }

            toplevel = toplevel.add_option(module_subcommand);
        }

        Some(toplevel)
    }

    async fn command_handle(
        &mut self,
        ctx: &Context,
        interaction: &CommandInteraction,
    ) -> Result<(), ModuleError> {
        let data = interaction.data.options();

        let module_subcommand = data.first().expect("failed to get module id");
        let module = get_module_by_id(module_subcommand.name)?;
        let mut module_config = module
            .get_config(interaction.guild_id.unwrap_or_default())
            .await?;

        let field = match &module_subcommand.value {
            ResolvedValue::SubCommandGroup(data) => data.first().unwrap(),
            _ => panic!(),
        };
        let fields = module.get_config_fields();
        let field_prototype = fields
            .get(field.name)
            .expect("failed to find field prototype");

        let field_data = match &field.value {
            ResolvedValue::SubCommand(field_data) => field_data.first(),
            _ => panic!("invalid value for field command"),
        };

        if field_data.is_none() {
            debug!("getting {}", field.name);
            let current = module_config
                .get_config_entry(field.name)
                .await
                .expect("failed to get field");
            if let Err(err) = interaction
                .edit_response(
                    ctx.http(),
                    EditInteractionResponse::new().content(format!(
                        "Current value: {}",
                        match field_prototype.field_type {
                            ConfigEntryType::User => format!("<@{}>", current.to_u64().unwrap()),
                            ConfigEntryType::Role => format!("<@&{}>", current.to_u64().unwrap()),
                            ConfigEntryType::ChannelText =>
                                format!("<#{}>", current.to_u64().unwrap()),
                            ConfigEntryType::U64 => current.to_u64().unwrap().to_string(),
                            ConfigEntryType::String => current.to_string().unwrap(),
                        }
                    )),
                )
                .await
            {
                warn!("failed to edit response: {err:?}");
            }
        } else {
            debug!("setting {}", field.name);
            if let Err(err) = module_config
                .set_config_entry(
                    field.name,
                    match field_prototype.field_type {
                        ConfigEntryType::U64
                        | ConfigEntryType::User
                        | ConfigEntryType::Role
                        | ConfigEntryType::ChannelText => {
                            ConfigValue::U64(match field_data.unwrap().value {
                                ResolvedValue::User(user, _) => user.id.get(),
                                ResolvedValue::Role(role) => role.id.get(),
                                ResolvedValue::Channel(channel) => channel.id.get(),
                                _ => panic!(),
                            })
                        }
                        ConfigEntryType::String => {
                            ConfigValue::String(match field_data.unwrap().value {
                                ResolvedValue::String(string) => string.to_string(),
                                _ => panic!(),
                            })
                        }
                    },
                )
                .await
            {
                if let Err(err2) = interaction
                    .edit_response(
                        ctx.http(),
                        EditInteractionResponse::new()
                            .content(format!("Failed to update config entry: {:?}", err)),
                    )
                    .await
                {
                    warn!("failed to edit response: {err2:?}");
                };
            } else {
                if let Err(err2) = module_config
                    .save(interaction.guild_id.unwrap_or_default())
                    .await
                {
                    warn!("failed to save config: {err2:?}");
                }
                if let Err(err2) = interaction
                    .edit_response(
                        ctx.http(),
                        EditInteractionResponse::new().content("Updated config entry."),
                    )
                    .await
                {
                    warn!("failed to edit response: {err2:?}");
                }
            }
        }

        Ok(())
    }
}

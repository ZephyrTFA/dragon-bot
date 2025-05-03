use super::ConfigManager;
use crate::{
    core::{
        commands::DragonModuleCommand,
        module::{DragonBotModule, get_module, get_module_by_id},
    },
    module::{errors::ModuleError, module_manager::ModuleManager},
};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, GuildId,
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
            .filter(|id| *id != "module-manager") // skip module-manager; it doesn't have a user facing config and will fail to get a handle because the module manager is what calls us
            .map(|id| get_module_by_id(id))
        {
            let module = module.unwrap();
            let mut module_subcommand = CreateCommandOption::new(
                CommandOptionType::SubCommandGroup,
                module.module_id(),
                format!("config for {}", module.module_id()),
            );

            let get_field_command = CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "get",
                "get a config entry",
            );
            let set_field_command = CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "set",
                "set a config entry",
            );

            for (_field, _field_data) in module.get_config_fields() {
                // TODO!
            }
            module_subcommand = module_subcommand
                .add_sub_option(get_field_command)
                .add_sub_option(set_field_command);
            toplevel = toplevel.add_option(module_subcommand);
        }

        Some(toplevel)
    }

    async fn command_handle(
        &mut self,
        _ctx: &Context,
        _interaction: &CommandInteraction,
    ) -> Result<(), ModuleError> {
        todo!()
    }
}

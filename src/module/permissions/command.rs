use super::PermissionsManager;
use crate::{
    core::{
        commands::DragonModuleCommand,
        module::{DragonBotModule, get_module_by_id},
        modules::DragonBotModuleInstance,
    },
    module::errors::ModuleError,
};
use log::warn;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
};

impl DragonModuleCommand for PermissionsManager {
    async fn command_builder(&self) -> Option<CreateCommand> {
        let mut builder = CreateCommand::new(self.id()).description("manage permissions");

        for module_id in DragonBotModuleInstance::all_module_ids() {
            let module = get_module_by_id(module_id)
                .await
                .expect("invalid module id");
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
            for permission in module.instance().all_permissions().await {
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
        _ctx: &Context,
        _interaction: &CommandInteraction,
    ) -> Result<(), ModuleError> {
        todo!()
    }
}

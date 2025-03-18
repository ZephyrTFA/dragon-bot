use super::{DragonBotModule, manager::module_manager, modules::DragonBotModuleInstance};
use serenity::{
    all::{Context, EventHandler, Interaction},
    async_trait,
};

pub struct ModuleEventHandler {}

#[async_trait]
impl EventHandler for ModuleEventHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let module_manager = module_manager().await;

        if let Interaction::Command(command) = &interaction {
            let module = module_manager.get_module_by_id(&command.data.name);
            if module.is_err() {
                return;
            }

            let module = module.unwrap();
            match module {
                DragonBotModuleInstance::ConfigManager(config_manager) => {
                    config_manager.command_handle(ctx, interaction).await;
                }
                DragonBotModuleInstance::ErrorManager(error_manager) => {
                    error_manager.command_handle(ctx, interaction).await;
                }
                DragonBotModuleInstance::PermissionsManager(permissions_manager) => {
                    permissions_manager.command_handle(ctx, interaction).await;
                }
                DragonBotModuleInstance::TgVerify(tg_verify) => {
                    tg_verify.command_handle(ctx, interaction).await;
                }
                DragonBotModuleInstance::TgDb(tg_db) => {
                    tg_db.command_handle(ctx, interaction).await;
                }
            }
        }
    }
}

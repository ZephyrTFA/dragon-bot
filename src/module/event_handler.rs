use super::{manager::module_manager, modules::DragonBotModuleInstance};
use serenity::{
    all::{Context, EventHandler, Guild, Interaction},
    async_trait,
};

pub struct ModuleEventHandler;

impl ModuleEventHandler {
    async fn create_command(&self, _guild: Guild, _module: DragonBotModuleInstance) {
        // let builder = module.command_builder();
    }
}

#[async_trait]
impl EventHandler for ModuleEventHandler {
    async fn ready(&self, _ctx: Context, _ready: serenity::model::gateway::Ready) {
        let _module_manager = module_manager().await;
        todo!()
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let module_manager = module_manager().await;

        if let Interaction::Command(command) = &interaction {
            let module = module_manager.get_module_by_id(&command.data.name);
            if module.is_err() {
                return;
            }

            let module = module.unwrap();
            module.command_handle(ctx, interaction).await;
        }
    }
}

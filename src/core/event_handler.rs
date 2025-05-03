use super::module::get_module_by_id_mut;
use crate::{
    core::{module::get_module, modules::DragonBotModuleInstance},
    module::module_manager::ModuleManager,
};
use log::{debug, info, warn};
use serenity::{
    all::{
        CacheHttp, Context, CreateInteractionResponseFollowup, EventHandler, Interaction, Ready,
    },
    async_trait,
};

pub struct ModuleEventHandler;

impl ModuleEventHandler {
    async fn init_modules(&self, ctx: &Context) {
        info!("Initializing modules...");

        for module in DragonBotModuleInstance::all_module_ids() {
            match get_module_by_id_mut(module).map(async |module| module.init(ctx).await) {
                Err(err) => warn!("failed to init {module}: {err:?}"),
                Ok(result) => {
                    if let Err(err) = result.await {
                        warn!("failed to init {module}: {err:?}");
                    }
                }
            }
        }

        info!("Modules initialized.");
    }

    async fn init_commands(&self, ctx: &Context) {
        for global in ctx.http().get_global_commands().await.iter().flatten() {
            if let Err(error) = ctx.http().delete_global_command(global.id).await {
                warn!("Failed to delete global command: {error}");
            } else {
                debug!("deleted global command: {}", global.name);
            }
        }
        if let Err(err) = Self::init_guild_commands(ctx).await {
            warn!("failed to initialize guild commands: {err:?}");
        }
    }
}

#[async_trait]
impl EventHandler for ModuleEventHandler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        self.init_modules(&ctx).await;
        self.init_commands(&ctx).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = &interaction {
            let guild = command.guild_id;
            if guild.is_none() {
                warn!("Attempted to run a global command. NYI.");
                return;
            }
            let guild = guild.unwrap();

            let module =
                get_module::<ModuleManager>().expect("failed to get module manager for reading");
            let module: &ModuleManager = module.module();

            let target_module = &command.data.name;
            let module_is_active = module.is_module_id_active(guild, target_module).await;
            match module_is_active {
                Ok(false) => {
                    warn!(
                        "Attempted to run a command for an inactive module {target_module}: {}",
                        command.data.name
                    );
                    return;
                }
                Err(err) => {
                    warn!("Failed to query module active state {target_module}: {err:?}",);
                    return;
                }
                _ => {}
            };

            let module = get_module_by_id_mut(&command.data.name);
            let command = interaction.command().unwrap();
            if let Err(error) = command.defer(ctx.http()).await {
                warn!("Failed to defer command: {error}");
                return;
            }

            let response = command.get_response(ctx.http()).await;
            if let Err(error) = &response {
                warn!("Failed to fetch defer response: {error}");
                return;
            }

            let module = module.unwrap();
            let result = module.command_handle(&ctx, &command).await;
            if let Err(error) = result {
                if let Err(error) = command
                    .create_followup(
                        ctx.http(),
                        CreateInteractionResponseFollowup::new()
                            .content(format!("Command failed: `{error:?}`"))
                            .ephemeral(true),
                    )
                    .await
                {
                    warn!("Failed to send error response to interaction: {error}");
                }
            }
        }
    }
}

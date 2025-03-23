use super::module::{
    DragonBotModule, GetModule, get_module_by_id, get_module_by_id_mut, get_module_mut,
};
use crate::{core::module::get_module, module::module_manager::ModuleManager};
use log::{debug, error, info, warn};
use serenity::{
    all::{
        CacheHttp, Context, CreateInteractionResponseFollowup, EventHandler, Interaction, Ready,
    },
    async_trait,
};
use std::process::exit;

pub struct ModuleEventHandler;

impl ModuleEventHandler {
    async fn init_modules(&self, ctx: &Context) {
        info!("Initializing modules...");
        let mut holder = get_module_mut::<ModuleManager>().await;
        let manager: &mut ModuleManager = holder.module_mut();
        if let Err(err) = manager.init(ctx).await {
            error!("Failed to initialize modules: {err:?}");
            exit(1);
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

            let manager = get_module::<ModuleManager>().await;
            let manager: &ModuleManager = manager.module();

            let target_module = &command.data.name;
            if !manager.is_module_id_active(guild, target_module) {
                warn!(
                    "Attempted to run a command for an inactive module {target_module}: {}",
                    command.data.name
                );
                return;
            }

            let module = get_module_by_id_mut(&command.data.name).await;
            if module.is_none() {
                warn!(
                    "Attempted to run an unknown interaction: {}",
                    command.data.name
                );
                return;
            }

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

            let mut module = module.unwrap();
            let result = module.instance_mut().command_handle(&ctx, &command).await;
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

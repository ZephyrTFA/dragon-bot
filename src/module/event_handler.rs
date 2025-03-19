use std::process::exit;

use super::{DragonBotModule, errors::ModuleError, manager::module_manager};
use log::{error, warn};
use serenity::{
    all::{Builder, CacheHttp, Context, EventHandler, GuildInfo, Http, Interaction, Ready},
    async_trait,
};

pub struct ModuleEventHandler;

#[derive(Debug)]
pub enum CommandError {
    Serenity(serenity::Error),
}

impl ModuleEventHandler {
    async fn init_guild_commands(&self, ctx: &Context) -> Result<(), ModuleError> {
        let manager = module_manager().await;

        for guild in Self::get_all_guilds(ctx.http()).await? {
            let current_commands = ctx
                .http()
                .as_ref()
                .get_guild_commands(guild.id)
                .await
                .map_err(CommandError::Serenity)?;
            let mut current_commands = current_commands
                .iter()
                .map(|v| (v.name.clone(), v))
                .collect::<Vec<_>>();

            let active_modules = manager.get_active_modules(guild.id);
            let wanted_commands = active_modules
                .iter()
                .filter_map(|module| module.command_builder());

            for new in
                wanted_commands.map(async |builder| builder.execute(ctx.http(), (None, None)).await)
            {
                let new = new.await;
                if let Err(e) = &new {
                    warn!("Failed to create command: {e}");
                    continue;
                }
                let new = new.unwrap();

                current_commands.retain(|(k, _)| *k != new.name);
            }
        }

        Ok(())
    }

    pub async fn get_all_guilds(http: impl AsRef<Http>) -> Result<Vec<GuildInfo>, ModuleError> {
        let http = http.as_ref();
        let mut guilds: Vec<GuildInfo> = vec![];
        loop {
            let mut fetched = http
                .get_guilds(
                    if guilds.is_empty() {
                        None
                    } else {
                        Some(serenity::all::GuildPagination::After(
                            guilds.last().unwrap().id,
                        ))
                    },
                    Some(100),
                )
                .await
                .map_err(CommandError::Serenity)?;

            guilds.append(&mut fetched);
            if fetched.len() < 100 {
                break;
            }
        }

        Ok(guilds)
    }
}

#[async_trait]
impl EventHandler for ModuleEventHandler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        if let Err(err) = module_manager().await.init(&ctx).await {
            error!("failed to initialize module manager: {err:?}");
            exit(1);
        }

        if let Err(err) = self.init_guild_commands(&ctx).await {
            warn!("failed to initialize guild commands: {err:?}");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let module_manager = module_manager().await;

        if let Interaction::Command(command) = &interaction {
            let module =
                module_manager.get_module_by_id(command.guild_id.unwrap(), &command.data.name);
            if module.is_err() {
                return;
            }

            let module = module.unwrap();
            module.command_handle(ctx, interaction).await;
        }
    }
}

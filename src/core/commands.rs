use super::{event_handler::ModuleEventHandler, modules::DragonBotModuleInstance};
use crate::{
    core::modules::get_module_instance_by_id,
    get_module,
    module::{errors::ModuleError, module_manager::ModuleManager},
    util::get_all_guilds,
};
use log::{debug, info, warn};
use serenity::all::{Builder, CacheHttp, CommandInteraction, Context, CreateCommand, GuildId};
use std::ops::Deref;

pub trait DragonModuleCommand {
    fn command_builder(&self) -> Option<CreateCommand> {
        None
    }

    fn command_handle(
        &mut self,
        _ctx: &Context,
        _command: &CommandInteraction,
    ) -> impl Future<Output = Result<(), ModuleError>> {
        async { Ok(()) }
    }

    fn command_help(
        &self,
        _ctx: &Context,
        _interaction: &CommandInteraction,
    ) -> impl Future<Output = ()> {
        async { todo!("default help handler") }
    }
}

impl ModuleEventHandler {
    pub async fn register_guild_module_command(
        ctx: &Context,
        guild: GuildId,
        module: &DragonBotModuleInstance,
    ) {
        let builder = module.command_builder();
        if builder.is_none() {
            return;
        }
        if let Err(error) = builder
            .unwrap()
            .execute(ctx.http(), (Some(guild), None))
            .await
        {
            warn!("Failed to register guild command: {error}");
        }
    }

    pub async fn drop_guild_module_command(
        ctx: &Context,
        guild: GuildId,
        module: &DragonBotModuleInstance,
    ) {
        let commands = ctx.http().get_guild_commands(guild).await;
        if let Err(error) = &commands {
            warn!("Failed to fetch guild commands: {error}");
            return;
        }

        if let Some(command) = commands
            .unwrap()
            .iter()
            .filter(|command| command.name == module.module_id())
            .nth(0)
        {
            if let Err(error) = ctx.http().delete_guild_command(guild, command.id).await {
                warn!("Failed to delete old command: {error}");
            }
        }
    }

    pub(super) async fn init_guild_commands(ctx: &Context) -> Result<(), ModuleError> {
        info!("Initializing guild commands");

        for guild in get_all_guilds(ctx).await? {
            debug!("init_guild_commands: {}", guild.id);

            let current_commands = ctx.http().get_guild_commands(guild.id).await;
            for command in current_commands.iter().flatten() {
                if let Err(error) = ctx.http().delete_guild_command(guild.id, command.id).await {
                    warn!("Failed to delete old command: {error}");
                }
            }

            get_module!(manager, instance, ModuleManager);
            let active_modules = manager.get_all_active_module_ids(guild.id);
            drop(instance);

            let mut wanted_commands = vec![];
            for active_module in active_modules {
                let module = get_module_instance_by_id(&active_module).await?;
                if let Some(command) = module.command_builder() {
                    debug!("wanted command: {:?}", command);
                    wanted_commands.push(command);
                }
            }

            let mut created_commands = vec![];
            for new in wanted_commands
                .into_iter()
                .map(async |builder| builder.execute(ctx.http(), (None, None)).await)
            {
                let new = new.await;
                if let Err(e) = &new {
                    warn!("Failed to create command: {e}");
                    continue;
                }

                created_commands.push(new.unwrap());
            }

            for command in created_commands {
                info!("Created: {}|{}", command.name, command.id);
            }
        }

        Ok(())
    }
}

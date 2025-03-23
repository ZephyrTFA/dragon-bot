use super::{event_handler::ModuleEventHandler, modules::DragonBotModuleInstance};
use crate::{
    core::module::{GetModule, get_module, get_module_by_id},
    module::{errors::ModuleError, module_manager::ModuleManager},
    util::get_all_guilds,
};
use log::{debug, info, warn};
use serenity::all::{Builder, CacheHttp, CommandInteraction, Context, CreateCommand, GuildId};

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
                } else {
                    debug!("deleted old command: {}", command.name);
                }
            }

            let mut wanted_commands = vec![];

            let manager = get_module::<ModuleManager>().await;
            let manager: &ModuleManager = manager.module();

            let active_modules = manager.get_all_active_module_ids(guild.id).clone();
            if let Some(manager_command) = manager.command_builder() {
                wanted_commands.push(manager_command);
            }

            debug!("getting wanted commands");
            for active_module in active_modules {
                debug!("checking: {active_module}");
                let module = get_module_by_id(&active_module).await;
                if module.is_none() {
                    warn!("invalid active module: {}", active_module);
                    continue;
                }
                let module = module.unwrap();

                if let Some(command) = module.instance().command_builder() {
                    debug!("wanted command: {:?}", command);
                    wanted_commands.push(command);
                }
                drop(module);
            }

            debug!("creating commands");
            let mut created_commands = vec![];
            for new in wanted_commands {
                debug!("creating...");
                let command = new.execute(ctx.http(), (Some(guild.id), None)).await;
                debug!("created!");
                if let Err(e) = &command {
                    warn!("Failed to create command: {e}");
                    continue;
                } else {
                    debug!("created: {}", command.as_ref().unwrap().name);
                    created_commands.push(command.unwrap());
                }
            }

            for command in created_commands {
                info!("Created: {}|{}", command.name, command.id);
            }
        }

        Ok(())
    }
}

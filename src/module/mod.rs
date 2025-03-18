use serenity::all::{Context, CreateCommand, Interaction};

pub mod config;
pub mod errors;
pub mod event_handler;
pub mod manager;
pub mod modules;
pub mod permissions;
pub mod tg_verify;
pub mod tgdb;

pub trait DragonBotModule
where
    Self: Default,
{
    fn module_id() -> &'static str
    where
        Self: Sized;

    fn id(&self) -> &'static str
    where
        Self: Sized,
    {
        Self::module_id()
    }

    fn init(&mut self) {}

    fn command_builder() -> Option<CreateCommand> {
        None
    }

    fn command_handle(&self, _ctx: Context, _interaction: Interaction) -> impl Future<Output = ()> {
        async {}
    }

    fn command_help(&self, _ctx: Context, _interaction: Interaction) -> impl Future<Output = ()> {
        async { todo!("default help handler") }
    }
}

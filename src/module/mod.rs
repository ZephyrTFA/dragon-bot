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
}

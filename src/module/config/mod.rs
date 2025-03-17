use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use serenity::all::GuildId;

use super::{DragonBotModule, errors::ModuleError};

pub trait Configurable<'d, C>
where
    Self: DragonBotModule + Send,
    C: Serialize + Deserialize<'d>,
{
    fn get_config(&self, _guild: GuildId) -> Result<ConfigHolder<'d, C>, ModuleError> {
        todo!()
    }

    fn set_config(&self, _guild: GuildId) -> Result<(), ModuleError> {
        todo!()
    }
}

pub struct ConfigHolder<'d, C>
where
    C: Serialize + Deserialize<'d>,
{
    config: C,
    guild: GuildId,
    phantom: PhantomData<&'d C>,
}

impl<'d, C> ConfigHolder<'d, C>
where
    C: Serialize + Deserialize<'d>,
{
    pub fn get(&self) -> &C {
        &self.config
    }

    pub fn get_mut(&mut self) -> &mut C {
        &mut self.config
    }

    pub fn guild(&self) -> GuildId {
        self.guild
    }
}

impl<'d, C> Drop for ConfigHolder<'d, C>
where
    C: Serialize + Deserialize<'d>,
{
    fn drop(&mut self) {}
}

#[derive(Default)]
pub struct ConfigManager {}

impl DragonBotModule for ConfigManager {
    fn module_id() -> &'static str
    where
        Self: Sized,
    {
        "config-manager"
    }
}

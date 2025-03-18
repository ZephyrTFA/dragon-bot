use log::error;
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use serenity::all::GuildId;

use super::{DragonBotModule, errors::ModuleError};

#[derive(Debug)]
pub enum ConfigError {
    SerdeError(serde_json::Error),
}

pub trait Configurable<'d, C>
where
    Self: DragonBotModule + Send,
    C: Serialize + Deserialize<'d>,
{
    fn get_config(&'d self, guild: GuildId) -> Result<ConfigHolder<'d, C, Self>, ModuleError> {
        Ok(ConfigHolder {
            config: serde_json::from_str("{}")
                .map_err(|e| ModuleError::ConfigError(ConfigError::SerdeError(e)))?,
            guild,
            owner: self,
            phantom: PhantomData {},
        })
    }

    fn set_config(&self, _guild: GuildId) -> Result<(), ModuleError> {
        todo!()
    }
}

pub struct ConfigHolder<'d, C, M>
where
    C: Serialize + Deserialize<'d>,
    M: Configurable<'d, C>,
{
    config: C,
    guild: GuildId,
    owner: &'d M,
    phantom: PhantomData<&'d C>,
}

impl<'d, C, M> ConfigHolder<'d, C, M>
where
    C: Serialize + Deserialize<'d>,
    M: Configurable<'d, C>,
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

impl<'d, C, M> Drop for ConfigHolder<'d, C, M>
where
    C: Serialize + Deserialize<'d>,
    M: Configurable<'d, C>,
{
    fn drop(&mut self) {
        if let Err(error) = self.owner.set_config(self.guild) {
            error!("failed to save config: {error:?}");
        };
    }
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

use std::{collections::HashMap, time::Duration};

mod config;

use mysql::{Pool, PooledConn};
use serenity::all::GuildId;

use super::{DragonBotModule, errors::ModuleError};

#[derive(Default)]
pub struct TgDb {
    pool: HashMap<GuildId, Option<Pool>>,
}

impl TgDb {
    pub fn get_conn(&self, guild: GuildId) -> Result<PooledConn, ModuleError> {
        Ok(self
            .pool
            .get(&guild)
            .ok_or(TgDbError::NotConnected)?
            .as_ref()
            .ok_or(TgDbError::NotConnected)?
            .try_get_conn(Duration::from_secs(5))
            .map_err(TgDbError::MysqlError)?)
    }
}

impl DragonBotModule for TgDb {
    fn module_id() -> &'static str
    where
        Self: Sized,
    {
        "tgdb"
    }
}

#[derive(Debug)]
pub enum TgDbError {
    MysqlError(mysql::Error),
    InternalError(String),
    NotConnected,
}

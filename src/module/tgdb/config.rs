use crate::module::config::DragonModuleConfigurable;

use super::TgDb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TgDbConfig {
    pub address: String,
    pub port: u64,
    pub user: String,
    pub password_b64: String,
    pub database: String,
}

impl DragonModuleConfigurable for TgDb {
    type Config = TgDbConfig;
}

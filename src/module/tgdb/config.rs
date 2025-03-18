use serde::{Deserialize, Serialize};

use crate::module::config::Configurable;

use super::TgDb;

#[derive(Serialize, Deserialize, Default)]
pub(super) struct TgDbConfig {
    pub address: String,
    pub port: u64,
    pub user: String,
    pub password_b64: String,
    pub database: String,
}

impl Configurable<TgDbConfig> for TgDb {}

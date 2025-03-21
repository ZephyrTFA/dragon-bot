use super::TgVerify;
use crate::module::config::Configurable;
use serde::{Deserialize, Serialize};
use serenity::all::RoleId;

#[derive(Serialize, Deserialize, Default)]
pub(super) struct TgVerifyConfig {
    pub role_verified_linked: RoleId,
    pub role_verified_living: RoleId,
    pub living_minutes_required: u32,
    pub discord_links_table: String,
}

impl Configurable<TgVerifyConfig> for TgVerify {}

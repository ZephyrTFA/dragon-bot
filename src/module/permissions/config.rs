use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serenity::all::{RoleId, UserId};

use crate::module::config::Configurable;

use super::PermissionsManager;

#[derive(Serialize, Deserialize)]
pub(super) struct PermissionsManagerConfig {
    pub user: HashMap<UserId, Vec<String>>,
    pub role: HashMap<RoleId, Vec<String>>,
}

impl Configurable<'_, PermissionsManagerConfig> for PermissionsManager {}

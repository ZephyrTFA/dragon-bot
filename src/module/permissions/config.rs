use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serenity::all::{RoleId, UserId};

use crate::module::config::Configurable;

use super::PermissionsManager;

#[derive(Serialize, Deserialize, Default)]
pub(super) struct PermissionsManagerConfig {
    pub user: HashMap<UserId, Vec<String>>,
    pub role: HashMap<RoleId, Vec<String>>,
}

impl Configurable<PermissionsManagerConfig> for PermissionsManager {}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serenity::all::GenericId;

use crate::module::config::Configurable;

use super::PermissionsManager;

#[derive(Serialize, Deserialize, Default)]
pub(super) struct PermissionsManagerConfig {
    pub namespaces: HashMap<String, HashMap<GenericId, Vec<String>>>,
}

impl Configurable<PermissionsManagerConfig> for PermissionsManager {}

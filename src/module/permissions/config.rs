use crate::module::config::DragonModuleConfigurable;
use serde::{Deserialize, Serialize};
use serenity::all::GenericId;
use std::collections::HashMap;

use super::PermissionsManager;

#[derive(Serialize, Deserialize, Default)]
pub struct PermissionsManagerConfig {
    pub namespaces: HashMap<String, HashMap<GenericId, Vec<String>>>,
}

impl DragonModuleConfigurable for PermissionsManager {
    type Config = PermissionsManagerConfig;
}

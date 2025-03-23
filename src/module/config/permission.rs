use super::ConfigManager;
use crate::core::permissions::{DragonModulePermissions, ModulePermission};

pub const EDIT_CONFIG: ModulePermission = ModulePermission::new(
    "config-manager",
    "edit-config",
    "update the configs for a guild's modules",
);

impl DragonModulePermissions for ConfigManager {
    async fn all_permissions(&self) -> Vec<ModulePermission> {
        vec![EDIT_CONFIG]
    }
}

use crate::core::permissions::{DragonModulePermission, ModulePermission};

use super::PermissionsManager;

pub const EDIT_PERMISSIONS: ModulePermission = ModulePermission::new(
    "permissions-manager",
    "edit-permissions",
    "edit the permission tree for a guild",
);

impl DragonModulePermission for PermissionsManager {
    async fn all_permissions(&self) -> Vec<ModulePermission> {
        vec![EDIT_PERMISSIONS]
    }
}

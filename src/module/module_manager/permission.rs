use super::ModuleManager;
use crate::core::permissions::{DragonModulePermissions, ModulePermission};

pub const PERMISSION_MODULE_ACTIVATE: ModulePermission =
    ModulePermission::new("module-manager", "module-activate", "activate a module");
pub const PERMISSION_MODULE_DEACTIVATE: ModulePermission =
    ModulePermission::new("module-manager", "module-deactivate", "deactivate a module");

impl DragonModulePermissions for ModuleManager {
    async fn all_permissions(&self) -> Vec<ModulePermission> {
        vec![PERMISSION_MODULE_ACTIVATE, PERMISSION_MODULE_DEACTIVATE]
    }
}

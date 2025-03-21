use crate::core::permissions::DragonModulePermissions;

use super::ModuleManager;

pub const PERMISSION_MODULE_ACTIVATE: &str = "module-activate";
pub const PERMISSION_MODULE_DEACTIVATE: &str = "module-deactivate";

impl DragonModulePermissions for ModuleManager {
    fn all_permissions(&self) -> &'static [&'static str] {
        &[PERMISSION_MODULE_ACTIVATE, PERMISSION_MODULE_DEACTIVATE]
    }
}

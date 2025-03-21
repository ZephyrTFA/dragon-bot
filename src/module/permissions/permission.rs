use crate::core::permissions::DragonModulePermissions;

use super::PermissionsManager;

pub const EDIT_PERMISSIONS: &str = "edit-permissions";

impl DragonModulePermissions for PermissionsManager {
    fn all_permissions(&self) -> &'static [&'static str] {
        &[EDIT_PERMISSIONS]
    }
}

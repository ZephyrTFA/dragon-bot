use crate::core::permissions::DragonModulePermissions;

use super::ConfigManager;

pub const EDIT_CONFIG: &str = "config-edit";

impl DragonModulePermissions for ConfigManager {
    fn all_permissions(&self) -> &'static [&'static str] {
        &[EDIT_CONFIG]
    }
}

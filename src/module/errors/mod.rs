use super::{manager::ModuleManagerError, permissions::PermissionsError, tgdb::TgDbError};

pub enum ModuleError {
    TgDbError(TgDbError),
    ModuleManagerError(ModuleManagerError),
    PermissionsError(PermissionsError),
}

macro_rules! impl_from {
    ($type: ident) => {
        impl From<$type> for ModuleError {
            fn from(value: $type) -> Self {
                Self::$type(value)
            }
        }
    };
}

impl_from!(TgDbError);
impl_from!(ModuleManagerError);
impl_from!(PermissionsError);

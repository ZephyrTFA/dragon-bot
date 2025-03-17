use super::{
    DragonBotModule, config::ConfigManager, permissions::PermissionsManager, tg_verify::TgVerify,
    tgdb::TgDb,
};

pub enum DragonBotModuleInstance {
    TgDb(TgDb),
    TgVerify(TgVerify),
    ConfigManager(ConfigManager),
    PermissionsManager(PermissionsManager),
}

macro_rules! impl_from {
    ($type: ident) => {
        impl<'a> From<&'a DragonBotModuleInstance> for &'a $type
        where
            $type: DragonBotModule,
        {
            fn from(value: &'a DragonBotModuleInstance) -> Self {
                match value {
                    DragonBotModuleInstance::$type(v) => v,
                    _ => panic!("grabbed wrong instance type"),
                }
            }
        }
    };

    ($type: ident) => {
        impl<'a> From<&'a mut DragonBotModuleInstance> for &'a mut $type
        where
            $type: DragonBotModule,
        {
            fn from(value: &'a DragonBotModuleInstance) -> Self {
                match value {
                    DragonBotModuleInstance::$type(v) => v,
                    _ => panic!("grabbed wrong instance type"),
                }
            }
        }
    };
}

impl_from!(TgDb);
impl_from!(TgVerify);
impl_from!(ConfigManager);
impl_from!(PermissionsManager);

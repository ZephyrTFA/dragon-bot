use super::{
    DragonBotModule, config::ConfigManager, errors::ErrorManager, permissions::PermissionsManager,
    tg_verify::TgVerify, tgdb::TgDb,
};
use serenity::all::{Context, CreateCommand, Interaction};

pub enum DragonBotModuleInstance {
    TgDb(TgDb),
    TgVerify(TgVerify),
    ConfigManager(ConfigManager),
    PermissionsManager(PermissionsManager),
    ErrorManager(ErrorManager),
}

macro_rules! impl_from {
    ( $( $type: ident ),+ ) => {
        $(
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

            impl<'a> From<&'a mut DragonBotModuleInstance> for &'a mut $type
            where
                $type: DragonBotModule,
            {
                fn from(value: &'a mut DragonBotModuleInstance) -> Self {
                    match value {
                        DragonBotModuleInstance::$type(v) => v,
                        _ => panic!("grabbed wrong instance type"),
                    }
                }
            }
        )+

        impl DragonBotModuleInstance {
            pub async fn command_handle(
                &self,
                ctx: Context,
                interaction: Interaction,
            ) {
                match self {
                    $(
                        DragonBotModuleInstance::$type(instance) => instance.command_handle(ctx, interaction).await,
                    )+
                }
            }

            pub fn command_builder(&self) -> Option<CreateCommand> {
                match self {
                    $(
                        DragonBotModuleInstance::$type(instance) => instance.command_builder(),
                    )+
                }
            }
        }
    };
}

impl_from!(
    TgDb,
    TgVerify,
    ConfigManager,
    PermissionsManager,
    ErrorManager
);

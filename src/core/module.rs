use super::{commands::DragonModuleCommand, permissions::DragonModulePermissions};
use crate::module::errors::ModuleError;
use serenity::all::Context;

pub trait DragonBotModule
where
    Self: Default + DragonModulePermissions + DragonModuleCommand,
{
    fn module_id() -> &'static str;
    fn id(&self) -> &'static str {
        Self::module_id()
    }

    fn init(&mut self, _ctx: &Context) -> impl Future<Output = Result<(), ModuleError>> {
        async { Ok(()) }
    }
}

macro_rules! impl_from {
    ( $( $type: ident ),+ ) => {
        $(
            impl<'a> From<&'a mut DragonBotModuleInstance> for &'a mut $type {
                fn from(value: &'a mut DragonBotModuleInstance) -> Self {
                    match value {
                        DragonBotModuleInstance::$type(v) => v,
                        _ => panic!(),
                    }
                }
            }

            impl<'a> From<&'a DragonBotModuleInstance> for &'a $type {
                fn from(value: &'a DragonBotModuleInstance) -> Self {
                    match value {
                        DragonBotModuleInstance::$type(v) => v,
                        _ => panic!(),
                    }
                }
            }
        )+

        impl DragonBotModuleInstance {
            pub async fn command_handle(
                &mut self,
                ctx: &Context,
                interaction: &CommandInteraction,
            ) -> Result<(), ModuleError> {
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

            pub fn module_id(&self) -> &'static str {
                match self {
                    $(
                        DragonBotModuleInstance::$type(_) => $type::module_id(),
                    )+
                }
            }

            pub fn all_module_ids() -> Vec<&'static str> {
                vec![
                    $(
                        $type::module_id(),
                    )+
                ]
            }
        }

        pub fn init_modules() -> HashMap<String, DragonBotModuleInstance> {
            static INIT_STATE: OnceCell<()> = OnceCell::const_new();
            if INIT_STATE.initialized() {
                panic!("attempt to init modules more than once!");
            }
            _ = INIT_STATE.set(());

            vec![
            $(
                DragonBotModuleInstance::$type($type::default()),
            )+
            ].into_iter().map(|m| (m.module_id().to_string(), m)).collect()
        }
    };
}

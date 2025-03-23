use std::{
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::OnceLock,
};

use super::{
    commands::DragonModuleCommand, modules::DragonBotModuleInstance,
    permissions::DragonModulePermissions,
};
use crate::module::errors::ModuleError;
use serenity::all::Context;
use tokio::sync::{Mutex, MutexGuard};

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
                let mut ids = vec![
                    $(
                        $type::module_id(),
                    )+
                ];
                ids.sort();
                ids
            }
        }
    };
}

static MODULES: OnceLock<HashMap<String, Mutex<DragonBotModuleInstance>>> = OnceLock::new();

pub struct DragonBotModuleHolder {
    guard: MutexGuard<'static, DragonBotModuleInstance>,
}

pub trait GetModule<'a, T> {
    fn module(&'a self) -> &'a T;
    fn module_mut(&'a mut self) -> &'a mut T;
}

impl<'a, T> GetModule<'a, T> for DragonBotModuleHolder
where
    T: DragonBotModule + 'a,
    &'a T: From<&'a DragonBotModuleInstance>,
    &'a mut T: From<&'a mut DragonBotModuleInstance>,
{
    fn module(&'a self) -> &'a T {
        let _: PhantomData<T>;
        self.guard.deref().into()
    }

    fn module_mut(&'a mut self) -> &'a mut T {
        self.guard.deref_mut().into()
    }
}

impl DragonBotModuleHolder {
    pub fn instance(&self) -> &DragonBotModuleInstance {
        &self.guard
    }

    pub fn instance_mut(&mut self) -> &mut DragonBotModuleInstance {
        &mut self.guard
    }
}

pub async fn get_module<T>() -> DragonBotModuleHolder
where
    T: DragonBotModule,
{
    get_module_by_id(T::module_id()).await.unwrap()
}

pub async fn get_module_by_id<'a>(module: &str) -> Option<DragonBotModuleHolder> {
    let modules = MODULES.get().expect("modules not init");
    let mutex = modules.get(module)?;
    let guard = mutex.lock().await;
    Some(DragonBotModuleHolder { guard })
}

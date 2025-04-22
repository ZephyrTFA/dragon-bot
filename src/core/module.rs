use super::{
    commands::DragonModuleCommand, modules::DragonBotModuleInstance,
    permissions::DragonModulePermission,
};
use crate::module::errors::ModuleError;
use log::debug;
use serenity::all::Context;
use std::{collections::HashMap, sync::OnceLock, time::Duration};
use strum::IntoEnumIterator;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub trait DragonBotModule
where
    Self: Default + DragonModulePermission + DragonModuleCommand,
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

            pub async fn command_builder(&self) -> Option<CreateCommand> {
                match self {
                    $(
                        DragonBotModuleInstance::$type(instance) => instance.command_builder().await,
                    )+
                }
            }

            pub async fn all_permissions(&self) -> Vec<ModulePermission> {
                match self {
                    $(
                        DragonBotModuleInstance::$type(instance) => instance.all_permissions().await,
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

pub struct DragonBotModuleHolder {
    read: Option<RwLockReadGuard<'static, DragonBotModuleInstance>>,
    write: Option<RwLockWriteGuard<'static, DragonBotModuleInstance>>,
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
        self.instance().into()
    }

    fn module_mut(&'a mut self) -> &'a mut T {
        self.instance_mut().into()
    }
}

impl DragonBotModuleHolder {
    pub fn instance(&self) -> &DragonBotModuleInstance {
        self.read.as_deref().or(self.write.as_deref()).unwrap()
    }

    pub fn instance_mut(&mut self) -> &mut DragonBotModuleInstance {
        self.write
            .as_deref_mut()
            .expect("mutable call to read holder")
    }
}

impl Drop for DragonBotModuleHolder {
    fn drop(&mut self) {
        debug!("dropped holder to: {}", self.instance().module_id());
    }
}

pub async fn get_module<T>(
    timeout: Option<Duration>,
) -> Result<DragonBotModuleHolder, GetModuleError>
where
    T: DragonBotModule,
{
    get_module_by_id(T::module_id(), timeout).await
}

pub async fn get_module_by_id(
    module: &str,
    timeout: Option<Duration>,
) -> Result<DragonBotModuleHolder, GetModuleError> {
    let modules = MODULES.get_or_init(init_modules);
    let rwlock = modules.get(module).ok_or(GetModuleError::ModuleNotFound)?;

    let read_lock = if let Some(timeout) = timeout {
        tokio::time::timeout(timeout, rwlock.read()).await.ok()
    } else {
        rwlock.try_read().ok()
    };
    if read_lock.is_none() {
        return Err(GetModuleError::GetModuleBlocked);
    }

    debug!("creating read holder to: {}", module);
    Ok(DragonBotModuleHolder {
        read: Some(read_lock.unwrap()),
        write: None,
    })
}

pub async fn get_module_mut<T>(
    timeout: Option<Duration>,
) -> Result<DragonBotModuleHolder, GetModuleError>
where
    T: DragonBotModule,
{
    get_module_by_id_mut(T::module_id(), timeout).await
}

#[derive(Debug)]
pub enum GetModuleError {
    ModuleNotFound,
    GetModuleBlocked,
}

pub async fn get_module_by_id_mut(
    module: &str,
    timeout: Option<Duration>,
) -> Result<DragonBotModuleHolder, GetModuleError> {
    let modules = MODULES.get_or_init(init_modules);
    let rwlock = modules.get(module).ok_or(GetModuleError::ModuleNotFound)?;

    let write_lock = if let Some(timeout) = timeout {
        tokio::time::timeout(timeout, rwlock.write()).await.ok()
    } else {
        rwlock.try_write().ok()
    };
    if write_lock.is_none() {
        return Err(GetModuleError::GetModuleBlocked);
    }

    debug!("creating write holder to: {}", module);
    Ok(DragonBotModuleHolder {
        write: Some(write_lock.unwrap()),
        read: None,
    })
}

static MODULES: OnceLock<HashMap<String, RwLock<DragonBotModuleInstance>>> = OnceLock::new();
fn init_modules() -> HashMap<String, RwLock<DragonBotModuleInstance>> {
    debug!("initializing modules");
    let mut map = HashMap::new();
    for module in DragonBotModuleInstance::iter() {
        let id = module.module_id().to_string();
        map.insert(id, RwLock::new(module));
    }
    debug!("done");
    map
}

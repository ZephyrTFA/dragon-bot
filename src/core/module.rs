#![allow(static_mut_refs)]

use super::{
    commands::DragonModuleCommand, modules::DragonBotModuleInstance,
    permissions::DragonModulePermission,
};
use crate::module::{config::DragonModuleConfigurable, errors::ModuleError};
use log::debug;
use serenity::all::Context;
use std::{collections::HashMap, sync::OnceLock};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub enum GetModuleError {
    ModuleNotFound,
    ModuleBlocked,
}

pub trait DragonBotModule
where
    Self: Default + DragonModulePermission + DragonModuleCommand + DragonModuleConfigurable,
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
            pub async fn init(&mut self, ctx: &Context) -> Result<(), ModuleError>{
                match self {
                    $(
                        DragonBotModuleInstance::$type(module) => module.init(ctx).await,
                    )+
                }
            }

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

            pub async fn command_builder(&self, guild: GuildId) -> Option<CreateCommand> {
                match self {
                    $(
                        DragonBotModuleInstance::$type(instance) => instance.command_builder(guild).await,
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

            pub fn get_config_fields(&self) -> HashMap<&'static str, ConfigField> {
                match self {
                    $(
                        DragonBotModuleInstance::$type(_) => $type::get_config_fields(),
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

            pub fn module<'a, T>(&'a self) -> &'a T
                where &'a T: From<&'a DragonBotModuleInstance>,
            {
                <&'a T>::from(self)
            }

            pub fn module_mut<'a, T>(&'a mut self) -> &'a mut T
                where &'a mut T: From<&'a mut DragonBotModuleInstance>,
            {
                <&'a mut T>::from(self)
            }
        }
    };
}

static mut MODULES: OnceLock<HashMap<String, DragonBotModuleInstance>> = OnceLock::new();
pub fn init_module_map() {
    unsafe {
        _ = MODULES.set(init_modules());
    }
}

fn init_modules() -> HashMap<String, DragonBotModuleInstance> {
    debug!("initializing modules");
    let mut map = HashMap::new();
    for module in DragonBotModuleInstance::iter() {
        let id = module.module_id().to_string();
        map.insert(id, module);
    }
    debug!("done");
    map
}

pub fn get_module_by_id(id: &str) -> Result<&'static DragonBotModuleInstance, GetModuleError> {
    unsafe {
        let modules = MODULES.get().unwrap();
        modules.get(id).ok_or(GetModuleError::ModuleNotFound)
    }
}

pub fn get_module<T: DragonBotModule>() -> Result<&'static DragonBotModuleInstance, GetModuleError>
{
    get_module_by_id(T::module_id())
}

pub fn get_module_mut<T: DragonBotModule>()
-> Result<&'static mut DragonBotModuleInstance, GetModuleError> {
    get_module_by_id_mut(T::module_id())
}

pub fn get_module_by_id_mut(
    id: &str,
) -> Result<&'static mut DragonBotModuleInstance, GetModuleError> {
    unsafe {
        let modules = MODULES.get_mut().unwrap();
        modules.get_mut(id).ok_or(GetModuleError::ModuleNotFound)
    }
}

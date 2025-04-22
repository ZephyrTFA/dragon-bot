use super::ErrorManager;
use crate::module::config::DragonModuleConfigurable;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct ErrorManagerConfig;

impl DragonModuleConfigurable for ErrorManager {
    type Config = ErrorManagerConfig;
}

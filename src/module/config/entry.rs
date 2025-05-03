use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ConfigFieldError {
    FieldNotFound,
    ValueWrongType,
    MalformedData,
    NotInit,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigEntry {
    value_type: ConfigEntryType,
    raw: String,
    #[serde(skip)]
    data: Option<ConfigValue>,
}

pub struct ConfigField {
    pub field_type: ConfigEntryType,
    pub description: String,
}

impl ConfigField {
    pub fn new(field_type: ConfigEntryType, description: impl Into<String>) -> Self {
        Self {
            field_type,
            description: description.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ConfigEntryType {
    String,
    U64,
    Role,
    User,
    ChannelText,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ConfigValue {
    String(String),
    U64(u64),
    Vec(Box<Vec<ConfigValue>>),
    HashMap(Box<HashMap<String, ConfigValue>>),
}

impl ConfigValue {
    pub fn to_string(self) -> Result<String, ConfigFieldError> {
        match self {
            ConfigValue::String(value) => Ok(value),
            _ => Err(ConfigFieldError::ValueWrongType),
        }
    }

    pub fn to_u64(self) -> Result<u64, ConfigFieldError> {
        match self {
            ConfigValue::U64(value) => Ok(value),
            _ => Err(ConfigFieldError::ValueWrongType),
        }
    }

    pub fn to_vec(self) -> Result<Vec<ConfigValue>, ConfigFieldError> {
        match self {
            ConfigValue::Vec(value) => Ok(*value),
            _ => Err(ConfigFieldError::ValueWrongType),
        }
    }

    pub fn to_map<T>(self) -> Result<HashMap<T, ConfigValue>, ConfigFieldError>
    where
        T: for<'de> Deserialize<'de> + Hash + Eq,
    {
        match self {
            ConfigValue::HashMap(value) => {
                let mut map = HashMap::with_capacity(value.len());
                for (k, v) in *value {
                    map.insert(
                        serde_json::from_str(&k).map_err(|_| ConfigFieldError::MalformedData)?,
                        v,
                    );
                }
                Ok(map)
            }
            _ => Err(ConfigFieldError::ValueWrongType),
        }
    }
}

impl ConfigEntry {
    pub fn value_type(&self) -> &ConfigEntryType {
        &self.value_type
    }

    pub fn value(&mut self) -> Option<&mut ConfigValue> {
        self.data.as_mut()
    }

    pub fn init_value(&mut self) -> Result<(), ConfigFieldError> {
        if self.data.is_some() {
            return Ok(());
        }
        match self.value_type {
            ConfigEntryType::String => self.data = Some(ConfigValue::String(self.raw.clone())),
            ConfigEntryType::U64
            | ConfigEntryType::Role
            | ConfigEntryType::User
            | ConfigEntryType::ChannelText => {
                self.data = Some(ConfigValue::U64(
                    self.raw
                        .parse()
                        .map_err(|_| ConfigFieldError::MalformedData)?,
                ))
            }
        }
        Ok(())
    }
}

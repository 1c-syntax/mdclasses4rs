use serde::{Deserialize, Serialize};

use crate::general::{GeneralInfo, Item, StringValue};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationRoot {
    #[serde(flatten)]
    pub general_info: GeneralInfo,
    #[serde(default)]
    pub synonym: Vec<Item>,
    pub comment: StringValue,
    pub contained_objects: Vec<ContainedObject>,
    pub configuration_extension_compatibility_mode: StringValue,
    pub default_run_mode: StringValue,
    pub use_purposes: Vec<StringValue>,
    pub script_variant: ScriptVariant,
    pub vendor: StringValue,
    pub version: StringValue,
    // pub used_mobile_application_functionalities: String // TODO кривая генерация в источниках
    pub default_language: StringValue,
    pub brief_information: Vec<Item>,
    pub detailed_information: Vec<Item>,
    #[serde(default = "default_data_lock_mode", skip_serializing_if = "is_data_lock_automatic")]
    pub data_lock_control_mode: DataLockControlMode,
    pub languages: Vec<Language>,
    #[serde(default)]
    pub common_modules: Vec<StringValue>,
    #[serde(default)]
    pub subsystems: Vec<StringValue>,
    #[serde(default)]
    pub session_parameters: Vec<StringValue>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainedObject {
    class_id: String,
    object_id: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    uuid: String,
    name: StringValue,
    synonym: Vec<Item>,
    language_code: StringValue,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ScriptVariant {
    #[serde(rename="$value")]
    body: ScriptLanguage,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct DataLockControlMode {
    #[serde(rename="$value")]
    body: DataLockMode,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ScriptLanguage {
    English(String),
    Russian(String)
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DataLockMode {
    Managed(String),
    AutomaticAndManaged(String),
    Automatic(String)
}

impl DataLockControlMode {
    pub fn default() -> Self {
        DataLockControlMode {
            body: DataLockMode::Automatic("Automatic".to_string())
        }
    }
}

pub fn is_data_lock_automatic(data_lock: &DataLockControlMode) -> bool {
    return data_lock.body == DataLockMode::Automatic("Automatic".to_string())
}

fn default_data_lock_mode() -> DataLockControlMode {
    DataLockControlMode::default()
}
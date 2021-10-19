use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use quick_xml::de::from_reader;
use serde::{Deserialize, Serialize};

use crate::general::{BooleanValue, GeneralInfo, MDObject, TypeQualifier, StringValue, Value, Item};
use crate::general::md_enum::{ChoiceOnHistoryOnInputValue, CreateOnInputValue, DataHistoryValue, FillCheckingValue, FullTextSearchValue, IndexingValue};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonAttribute {
    #[serde(flatten)]
    pub general_info: GeneralInfo,
    #[serde(default)]
    pub synonym: Vec<Item>,
    #[serde(rename = "type", default)]
    pub types: Vec<TypeQualifier>,
    pub password_mode: Option<BooleanValue>,
    #[serde(rename = "type", default)]
    pub tool_tip: Vec<Item>,
    pub mark_negatives: Option<BooleanValue>,
    pub mask: Option<StringValue>,
    pub multi_line: Option<BooleanValue>,
    pub extended_edit: Option<BooleanValue>,
    pub min_value: Option<Value>,
    pub max_value: Option<Value>,
    #[serde(default)]
    pub fill_checking: FillCheckingValue,
    #[serde(default)]
    pub create_on_input: CreateOnInputValue,
    #[serde(default)]
    pub data_history: DataHistoryValue,
    pub fill_from_filling_value: Option<BooleanValue>,
    pub fill_value: Option<Value>,
    #[serde(default)]
    pub choice_history_on_input: ChoiceOnHistoryOnInputValue,
    #[serde(default)]
    pub auto_use: AutoUseValue,
    #[serde(default)]
    pub data_separation: DataSeparationValue,
    #[serde(default)]
    pub users_separation: UsersSeparationValue,
    #[serde(default)]
    pub authentication_separation: AuthenticationSeparationValue,
    #[serde(default)]
    pub configuration_extensions_separation: ConfigurationExtensionsSeparationValue,
    #[serde(default)]
    pub indexing: IndexingValue,
    #[serde(default)]
    pub full_text_search: FullTextSearchValue,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoUseValue {
    #[serde(rename = "$value")]
    pub value: AutoUse,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum AutoUse {
    DontUse,
    Use,
}

impl Default for AutoUseValue {
    fn default() -> Self {
        AutoUseValue {
            value: AutoUse::Use,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSeparationValue {
    #[serde(rename = "$value")]
    pub value: DataSeparation,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum DataSeparation {
    DontUse,
    Separate,
}

impl Default for DataSeparationValue {
    fn default() -> Self {
        DataSeparationValue {
            value: DataSeparation::Separate,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersSeparationValue {
    #[serde(rename = "$value")]
    pub value: UserSeparation,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum UserSeparation {
    DontUse,
    Separate,
}

impl Default for UsersSeparationValue {
    fn default() -> Self {
        UsersSeparationValue {
            value: UserSeparation::Separate,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationSeparationValue {
    #[serde(rename = "$value")]
    pub value: AuthenticationSeparation,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum AuthenticationSeparation {
    DontUse,
    Separate,
}

impl Default for AuthenticationSeparationValue {
    fn default() -> Self {
        AuthenticationSeparationValue {
            value: AuthenticationSeparation::Separate,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationExtensionsSeparationValue {
    #[serde(rename = "$value")]
    pub value: ConfigurationExtensionsSeparation,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ConfigurationExtensionsSeparation {
    DontUse,
    Separate,
}

impl Default for ConfigurationExtensionsSeparationValue {
    fn default() -> Self {
        ConfigurationExtensionsSeparationValue {
            value: ConfigurationExtensionsSeparation::Separate,
        }
    }
}

impl MDObject for CommonAttribute {
    fn new(root_path: &Path, attribute_name: String) -> Self {
        let attribute_name = attribute_name.to_owned();

        let mut path = PathBuf::new();
        path.push(root_path);
        path.push("CommonAttributes");
        path.push(&attribute_name);
        path.push(&attribute_name);
        path.set_extension("mdo");

        let file = File::open(path).unwrap();
        let file = BufReader::new(file);
        from_reader(file).unwrap()
    }
}

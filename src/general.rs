use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralInfo {
    #[serde(rename = "xmlns:mdclass")]
    pub md_class: String,
    pub uuid: String,
    pub name: StringValue,
    pub comment: Option<StringValue>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct StringValue {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Item {
    key: StringValue,
    value: StringValue,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BooleanValue {
    #[serde(rename = "$value")]
    value: bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct IntValue {
    #[serde(rename = "$value")]
    value: i32,
}

pub trait MDObject {
    fn new(root_path: &Path, name: String) -> Self;
}

pub fn read_objects<T: MDObject>(object_names: &Vec<StringValue>, root_path: &Path) -> Vec<T> {
    object_names.iter()
        .map(|full_name| &full_name.value)
        .map(|name| extract_name_from_full_name(name))
        .filter(|name| !name.is_empty())
        .map(|name| T::new(root_path, name))
        .collect()
}

fn extract_name_from_full_name(full_name: &String) -> String {
    return if let Some(split) = full_name.split_once(".") {
        split.1.to_string()
    } else {
        full_name.to_string()
    };
}

impl BooleanValue {
    pub fn new() -> Self {
        BooleanValue {
            value: false
        }
    }
}

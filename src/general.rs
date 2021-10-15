use std::path::Path;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
pub mod general_enum;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralInfo {
    #[serde(rename = "xmlns:mdclass")]
    md_class: String,
    pub uuid: String,
    pub name: StringValue,
    pub comment: Option<StringValue>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct StringValue {
    #[serde(rename = "$value")]
    #[serde(default)]
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

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterQualifier {
    #[serde(default)]
    pub length: i32,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberQualifier {
    #[serde(default)]
    pub precision: i32,
    #[serde(default)]
    pub scale: i32,
    #[serde(default)]
    pub non_negative: bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectType {
    pub types: Vec<StringValue>,
    pub string_qualifiers: Option<ParameterQualifier>,
    pub binary_qualifiers: Option<ParameterQualifier>,
    pub number_qualifiers: Option<NumberQualifier>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    #[serde(rename = "xsi:type")]
    xsi_type: String,
    #[serde(rename = "$value")]
    pub value: Option<StringValue>,
}

pub trait MDObject {
    fn new(root_path: &Path, name: String) -> Self;
}

pub fn read_objects<T: MDObject + std::marker::Send>(object_names: &Vec<StringValue>, root_path: &Path) -> Vec<T> {
    object_names.into_par_iter()
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

use serde::{Deserialize, Serialize};
use crate::general::{BooleanValue, Item, StringValue, TypeQualifier, Value};
use crate::general::md_enum::{DataHistoryValue, FullTextSearchValue, IndexingValue};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub uuid: String,
    pub name: StringValue,
    #[serde(default)]
    pub synonym: Vec<Item>,
    #[serde(rename = "type")]
    pub types: Vec<TypeQualifier>,
    pub password_mode: Option<BooleanValue>,
    #[serde(default)]
    pub tool_tip: Vec<Item>,
    pub mask: Option<StringValue>,
    pub multi_line: Option<BooleanValue>,
    pub extended_edit: Option<BooleanValue>,
    pub min_value: Option<Value>,
    pub max_value: Option<Value>,
    pub fill_value: Option<Value>,
    #[serde(default)]
    pub indexing: IndexingValue,
    #[serde(default)]
    pub full_text_search: FullTextSearchValue,
    #[serde(default)]
    pub data_history: DataHistoryValue,
}
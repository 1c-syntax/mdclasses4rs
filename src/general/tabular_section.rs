use serde::{Deserialize, Serialize};
use crate::general::produced_type::ProducedType;
use crate::general::{Item, StringValue};
use crate::general::attribute::Attribute;
use crate::general::md_enum::FillCheckingValue;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TabularSection {
    pub uuid: String,
    produced_types: ProducedType,
    pub name: StringValue,
    #[serde(default)]
    pub synonym: Vec<Item>,
    pub comment: Option<StringValue>,
    #[serde(default)]
    pub tool_tip: Vec<Item>,
    #[serde(default)]
    pub fill_checking: FillCheckingValue,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
}
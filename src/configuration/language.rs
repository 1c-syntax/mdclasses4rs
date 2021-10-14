use crate::general::{Item, StringValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    uuid: String,
    name: StringValue,
    synonym: Vec<Item>,
    language_code: StringValue,
}
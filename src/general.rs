use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct StringValue {
    #[serde(rename="$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Item {
    key: StringValue,
    value: StringValue,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BooleanValue {
    #[serde(rename="$value")]
    body: bool,
}
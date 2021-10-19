use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProducedType {
    object_type: Option<ObjectType>,
    row_type: Option<RowType>,
    ref_type: Option<RefType>,
    selection_type: Option<SelectionType>,
    list_type: Option<ListType>,
    manager_type: Option<ManagerType>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct ObjectType {
    type_id: String,
    value_type_id: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct RowType {
    type_id: String,
    value_type_id: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct RefType {
    type_id: String,
    value_type_id: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct SelectionType {
    type_id: String,
    value_type_id: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct ListType {
    type_id: String,
    value_type_id: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct ManagerType {
    type_id: String,
    value_type_id: String,
}
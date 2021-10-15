use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct FillCheckingValue {
    #[serde(rename = "$value")]
    pub value: FillChecking,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum FillChecking {
    DontCheck,
    ShowError,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOnInputValue {
    #[serde(rename = "$value")]
    pub value: CreateOnInput,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum CreateOnInput {
    Auto,
    DontUse,
    Use,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataHistoryValue {
    #[serde(rename = "$value")]
    pub value: DataHistory,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum DataHistory {
    DontUse,
    Use,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceOnHistoryOnInputValue {
    #[serde(rename = "$value")]
    pub value: ChoiceOnHistoryOnInput,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ChoiceOnHistoryOnInput {
    Auto,
    DontUse,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexingValue {
    #[serde(rename = "$value")]
    pub value: Indexing,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Indexing {
    DontIndex,
    Index,
    IndexWithAdditionalOrder,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct FullTextSearchValue {
    #[serde(rename = "$value")]
    pub value: FullTextSearch,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum FullTextSearch {
    DontUse,
    Use,
}

impl Default for DataHistoryValue {
    fn default() -> Self {
        DataHistoryValue {
            value: DataHistory::DontUse,
        }
    }
}

impl Default for FillCheckingValue {
    fn default() -> Self {
        FillCheckingValue {
            value: FillChecking::DontCheck,
        }
    }
}

impl Default for CreateOnInputValue {
    fn default() -> Self {
        CreateOnInputValue {
            value: CreateOnInput::Auto,
        }
    }
}

impl Default for ChoiceOnHistoryOnInputValue {
    fn default() -> Self {
        ChoiceOnHistoryOnInputValue {
            value: ChoiceOnHistoryOnInput::Auto,
        }
    }
}

impl Default for IndexingValue {
    fn default() -> Self {
        IndexingValue {
            value: Indexing::DontIndex,
        }
    }
}

impl Default for FullTextSearchValue {
    fn default() -> Self {
        FullTextSearchValue {
            value: FullTextSearch::DontUse,
        }
    }
}
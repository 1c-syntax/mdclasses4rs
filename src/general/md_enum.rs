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

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct FullTextSearchOnInputByStringValue {
    #[serde(rename = "$value")]
    pub value: FullTextSearchOnInputByString,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum FullTextSearchOnInputByString {
    DontUse,
    Use,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AllowedLengthValue {
    #[serde(rename = "$value")]
    pub value: AllowedLength,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum AllowedLength {
    Fixed,
    Variable,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct DefaultPresentationValue {
    #[serde(rename = "$value")]
    pub value: DefaultPresentation,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum DefaultPresentation {
    AsDescription,
    AsCode,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct EditTypeValue {
    #[serde(rename = "$value")]
    pub value: EditType,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum EditType {
    BothWays,
    InDialog,
    InList,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ChoiceModeValue {
    #[serde(rename = "$value")]
    pub value: ChoiceMode,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ChoiceMode {
    BothWays,
    FromForm,
    QuickChoice,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ChoiceDataGetModeOnInputByStringValue {
    #[serde(rename = "$value")]
    pub value: ChoiceDataGetModeOnInputByString,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ChoiceDataGetModeOnInputByString {
    Directly,
    Background,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SearchStringModeOnInputByStringValue {
    #[serde(rename = "$value")]
    pub value: SearchStringModeOnInputByString,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum SearchStringModeOnInputByString {
    Begin,
    AnyPart,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ChoiceHistoryOnInputValue {
    #[serde(rename = "$value")]
    pub value: ChoiceHistoryOnInput,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ChoiceHistoryOnInput {
    Auto,
    DontUse,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct DataHistoryUseValue {
    #[serde(rename = "$value")]
    pub value: DataHistoryUse,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum DataHistoryUse {
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

impl Default for FullTextSearchOnInputByStringValue {
    fn default() -> Self {
        FullTextSearchOnInputByStringValue {
            value: FullTextSearchOnInputByString::Use,
        }
    }
}

impl Default for AllowedLengthValue {
    fn default() -> Self {
        AllowedLengthValue {
            value: AllowedLength::Fixed
        }
    }
}

impl Default for DefaultPresentationValue {
    fn default() -> Self {
        DefaultPresentationValue {
            value: DefaultPresentation::AsCode
        }
    }
}

impl Default for EditTypeValue {
    fn default() -> Self {
        EditTypeValue {
            value: EditType::BothWays
        }
    }
}

impl Default for ChoiceModeValue {
    fn default() -> Self {
        ChoiceModeValue {
            value: ChoiceMode::FromForm
        }
    }
}

impl Default for SearchStringModeOnInputByStringValue {
    fn default() -> Self {
        SearchStringModeOnInputByStringValue {
            value: SearchStringModeOnInputByString::Begin
        }
    }
}

impl Default for ChoiceDataGetModeOnInputByStringValue {
    fn default() -> Self {
        ChoiceDataGetModeOnInputByStringValue {
            value: ChoiceDataGetModeOnInputByString::Directly
        }
    }
}

impl Default for ChoiceHistoryOnInputValue {
    fn default() -> Self {
        ChoiceHistoryOnInputValue {
            value: ChoiceHistoryOnInput::Auto
        }
    }
}

impl Default for DataHistoryUseValue {
    fn default() -> Self {
        DataHistoryUseValue {
            value: DataHistoryUse::DontUse
        }
    }
}
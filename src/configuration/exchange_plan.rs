use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use quick_xml::de::from_reader;
use serde::{Deserialize, Serialize};
use crate::configuration::configuration_root::DataLockControlValue;
use crate::general::{BooleanValue, GeneralInfo, IntValue, Item, MDObject, StringValue};
use crate::general::attribute::Attribute;
use crate::general::md_enum::{AllowedLengthValue, ChoiceDataGetModeOnInputByStringValue, ChoiceHistoryOnInputValue, ChoiceModeValue, CreateOnInputValue, DataHistoryUseValue, DefaultPresentationValue, FullTextSearchOnInputByStringValue, FullTextSearchValue, SearchStringModeOnInputByStringValue};
use crate::general::produced_type::ProducedType;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangePlan {
    #[serde(flatten)]
    pub general_info: GeneralInfo,
    #[serde(rename = "xmlns:xsi")]
    xsi: String,
    #[serde(rename = "xmlns:core")]
    core: String,
    this_node: String,
    produced_types: ProducedType,
    /// Синоним
    pub synonym: Vec<Item>,
    /// Использовать стандартные команды
    pub use_standard_commands: Option<BooleanValue>,
    #[serde(default)]
    /// Ввод по строке
    pub input_by_string: Vec<StringValue>,
    #[serde(default)]
    /// Способ поиска строки при вводе по строке
    pub search_string_mode_on_input_by_string: SearchStringModeOnInputByStringValue,
    #[serde(default)]
    /// Полнотекстовый поиск при вводе по строке
    pub full_text_search_on_input_by_string: FullTextSearchOnInputByStringValue,
    #[serde(default)]
    /// Режим получения данных выбора при вводе по строке
    pub choice_data_get_mode_on_input_by_string: ChoiceDataGetModeOnInputByStringValue,
    #[serde(default)]
    /// Вводится на основании
    pub based_on: Vec<StringValue>,
    #[serde(default)]
    /// Создание при вводе
    pub create_on_input: CreateOnInputValue,
    /// Включать в соджержимое справки
    pub include_help_in_contents: Option<BooleanValue>,
    #[serde(default)]
    /// Поля блокировки данных
    pub data_lock_fields: Vec<StringValue>,
    #[serde(default)]
    /// Режим управления блокировкой данных
    pub data_lock_control_mode: DataLockControlValue,
    #[serde(default)]
    /// Полнотекстовый поиск
    pub full_text_search: FullTextSearchValue,
    #[serde(default)]
    /// Представление объекта
    pub object_presentation: Vec<Item>,
    #[serde(default)]
    /// Расишренное представление объекта
    pub extended_object_presentation: Vec<Item>,
    #[serde(default)]
    /// Представление списка
    pub list_presentation: Vec<Item>,
    #[serde(default)]
    /// Расширенное представление списка
    pub extended_list_presentation: Vec<Item>,
    #[serde(default)]
    /// Пояснение
    pub explanation: Vec<Item>,
    #[serde(default)]
    /// История данных
    pub data_history: DataHistoryUseValue,
    /// Длина кода
    pub code_length: Option<IntValue>,
    #[serde(default)]
    /// Допустимая длина кода
    pub code_allowed_length: AllowedLengthValue,
    /// Длина наименования
    pub description_length: Option<IntValue>,
    #[serde(default)]
    /// Состав плана обмена
    pub content: Vec<PlanContent>,
    #[serde(default)]
    /// Основное представление плана обмена
    pub default_presentation: DefaultPresentationValue,
    /// Быстрый выбор
    pub quick_choice: Option<BooleanValue>,
    #[serde(default)]
    /// Способ выбора
    pub choice_mode: ChoiceModeValue,
    #[serde(default)]
    /// История выбора при вводе
    pub choice_history_on_input: ChoiceHistoryOnInputValue,
    /// Обновлять историю данных сразу после записи
    pub update_data_history_immediately_after_write: Option<BooleanValue>,
    /// Выполнять обработку после записи версии истории данных
    pub execute_after_write_data_history_version_processing: Option<BooleanValue>,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct PlanContent {
    /// Объект
    pub md_object: Option<StringValue>,
    #[serde(default)]
    /// Авторегистрация
    pub auto_record: AutoRegistrationChangesValue,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AutoRegistrationChangesValue {
    #[serde(rename = "$value")]
    pub value: AutoRegistrationChanges,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum AutoRegistrationChanges {
    Allow,
    Deny,
}

impl Default for AutoRegistrationChangesValue {
    fn default() -> Self {
        AutoRegistrationChangesValue {
            value: AutoRegistrationChanges::Deny
        }
    }
}

impl MDObject for ExchangePlan {
    fn new(root_path: &Path, exchange_plan_name: String) -> Self {
        let exchange_plan_name = exchange_plan_name.to_owned();

        let mut path = PathBuf::new();
        path.push(root_path);
        path.push("ExchangePlans");
        path.push(&exchange_plan_name);
        path.push(&exchange_plan_name);
        path.set_extension("mdo");

        let file = File::open(path).unwrap();
        let file = BufReader::new(file);
        from_reader(file).unwrap()
    }
}
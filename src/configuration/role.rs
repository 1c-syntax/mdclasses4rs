use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use quick_xml::de::from_reader;
use crate::general::{BooleanValue, GeneralInfo, Item, MDObject, StringValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    #[serde(flatten)]
    pub general_info: GeneralInfo,
    pub synonym: Vec<Item>,
    #[serde(skip)]
    pub rights_data: RightsData,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RightsData {
    pub set_for_new_objects: BooleanValue,
    pub set_for_attributes_by_default: BooleanValue,
    pub independent_rights_of_child_objects: BooleanValue,
    #[serde(rename = "object", default)]
    pub object_rights: Vec<RightInfo>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RightInfo {
    pub name: StringValue,
    #[serde(rename = "right", default)]
    pub right_value: Vec<RightValue>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RightValue {
    pub name: Right,
    pub value: BooleanValue,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Right {
    #[serde(rename = "$value")]
    name: RightName,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum RightName {
    ActiveUsers,
    Administration,
    AllFunctionsMode,
    AnalyticsSystemClient,
    Automation,
    CollaborationSystemInfoBaseRegistration,
    ConfigurationExtensionsAdministration,
    DataAdministration,
    Delete,
    Edit,
    EditDataHistoryVersionComment,
    EventLog,
    ExclusiveMode,
    ExclusiveModeTerminationAtSessionStart,
    Execute,
    ExternalConnection,
    Get,
    InputByString,
    Insert,
    InteractiveActivate,
    InteractiveChangeOfPosted,
    InteractiveClearDeletionMark,
    InteractiveClearDeletionMarkPredefinedData,
    InteractiveDelete,
    InteractiveDeleteMarked,
    InteractiveDeleteMarkedPredefinedData,
    InteractiveDeletePredefinedData,
    InteractiveExecute,
    InteractiveInsert,
    InteractiveOpenExtDataProcessors,
    InteractiveOpenExtReports,
    InteractivePosting,
    InteractivePostingRegular,
    InteractiveSetDeletionMark,
    InteractiveSetDeletionMarkPredefinedData,
    InteractiveStart,
    InteractiveUndoPosting,
    MainWindowModeEmbeddedWorkplace,
    MainWindowModeFullscreenWorkplace,
    MainWindowModeKiosk,
    MainWindowModeNormal,
    MainWindowModeWorkplace,
    MobileClient,
    Output,
    Posting,
    Read,
    ReadDataHistory,
    ReadDataHistoryOfMissingData,
    SaveUserData,
    SessionOsAuthenticationChange,
    SessionStandardAuthenticationChange,
    Set,
    StandardAuthenticationChange,
    Start,
    SwitchToDataHistoryVersion,
    TechnicalSpecialistMode,
    ThickClient,
    ThinClient,
    TotalsControl,
    UndoPosting,
    Update,
    UpdateDataBaseConfiguration,
    UpdateDataHistory,
    UpdateDataHistoryOfMissingData,
    UpdateDataHistorySettings,
    UpdateDataHistoryVersionComment,
    Use,
    View,
    ViewDataHistory,
    WebClient,
}

impl Default for RightsData {
    fn default() -> Self {
        RightsData {
            set_for_new_objects: BooleanValue::new(),
            set_for_attributes_by_default: BooleanValue::new(),
            independent_rights_of_child_objects: BooleanValue::new(),
            object_rights: Vec::new(),
        }
    }
}

impl MDObject for Role {
    fn new(root_path: &Path, role_name: String) -> Self {
        let role_name = role_name.to_owned();

        let mut path_to_role_folder = PathBuf::new();
        path_to_role_folder.push(root_path);
        path_to_role_folder.push("Roles");
        path_to_role_folder.push(&role_name);

        let mut path_to_role = PathBuf::from(&path_to_role_folder);
        path_to_role.push(&role_name);
        path_to_role.set_extension("mdo");

        let file = File::open(&path_to_role).unwrap();
        let file = BufReader::new(file);
        let mut role: Role = from_reader(file).unwrap();

        role.rights_data = read_rights_data(&path_to_role_folder);

        return role;
    }
}

fn read_rights_data(path_to_role_folder: &PathBuf) -> RightsData {
    let mut path_to_rights = PathBuf::from(path_to_role_folder);
    path_to_rights.push("Rights");
    path_to_rights.set_extension("rights");

    let file = File::open(&path_to_rights).unwrap();
    let file = BufReader::new(file);

    let rights_data: RightsData = from_reader(file).unwrap();
    return rights_data;
}
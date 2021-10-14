use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use quick_xml::de::from_reader;
use crate::general::GeneralInfo;
use crate::general;
use crate::general::{BooleanValue, Item, MDObject, StringValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subsystem {
    #[serde(flatten)]
    pub general_info: GeneralInfo,
    #[serde(default)]
    pub synonym: Vec<Item>,
    pub include_help_in_contents: Option<BooleanValue>,
    pub include_in_command_interface: Option<BooleanValue>,
    #[serde(default)]
    pub content: Vec<StringValue>,
    #[serde(default)]
    pub subsystems: Vec<StringValue>,
    #[serde(skip)]
    pub child_systems: Vec<Subsystem>,
}

impl MDObject for Subsystem {
    fn new(root_path: &Path, subsystem_name: String) -> Self {
        let full_subsystem_name = subsystem_name.to_owned();

        let mut path_to_parent_subsystem = PathBuf::new();
        path_to_parent_subsystem.push(root_path);
        path_to_parent_subsystem.push("Subsystems");
        path_to_parent_subsystem.push(&full_subsystem_name);

        let mut path_to_subsystem_mdo = PathBuf::from(&path_to_parent_subsystem);
        path_to_subsystem_mdo.push(&full_subsystem_name);
        path_to_subsystem_mdo.set_extension("mdo");

        let file = File::open(path_to_subsystem_mdo).unwrap();
        let file = BufReader::new(file);
        let mut subsystem: Subsystem = from_reader(file).unwrap();

        subsystem.child_systems = general::read_objects(&subsystem.subsystems, path_to_parent_subsystem.as_path());
        return subsystem;
    }
}
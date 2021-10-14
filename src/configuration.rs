use crate::configuration::configuration_root::{ConfigurationRoot, GeneralInfo};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;
use quick_xml::de::from_reader;
use std::ffi::OsStr;
use serde::{Deserialize, Serialize};
use crate::general::{StringValue, Item, BooleanValue};

pub mod configuration_root;

#[derive(Debug)]
pub struct Configuration {
    pub root: ConfigurationRoot,
    pub common_modules: Vec<CommonModule>,
    pub subsystems: Vec<Subsystem>,
}

trait MDObject {
    fn new(root_path: &Path, name: String) -> Self;
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonModule {
    #[serde(flatten)]
    pub general_info: GeneralInfo,
    #[serde(default)]
    pub synonym: Vec<Item>,
    pub global: Option<BooleanValue>,
    pub client_managed_application: Option<BooleanValue>,
    pub server: Option<BooleanValue>,
    pub external_connection: Option<BooleanValue>,
    pub client_ordinary_application: Option<BooleanValue>,
    pub server_call: Option<BooleanValue>,
    pub privileged: Option<BooleanValue>,
    pub return_values_reuse: Option<ReturnValueReuse>,
}

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

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ReturnValueReuse {
    #[serde(rename = "$value")]
    pub body: ValueReuse,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ValueReuse {
    DuringRequest(String),
    DuringSession(String),
}

impl Configuration {
    pub fn new<T: AsRef<Path>>(project_path: T) -> Self {
        let root_path = project_path.as_ref();
        let path: PathBuf = [root_path.as_os_str(), OsStr::new("Configuration"), OsStr::new("Configuration.mdo")].iter().collect();
        let file = File::open(path).unwrap();
        let file = BufReader::new(file);
        let root: ConfigurationRoot = from_reader(file).unwrap();

        let common_modules_names = &root.common_modules;
        let subsystems_names = &root.subsystems;

        let common_modules = read_objects(common_modules_names, root_path);
        let subsystems = read_objects(subsystems_names, root_path);

        Configuration {
            root,
            common_modules,
            subsystems,
        }
    }
}

impl MDObject for CommonModule {
    fn new(root_path: &Path, module_name: String) -> Self {
        let full_module_name = module_name.to_owned();

        let mut path = PathBuf::new();
        path.push(root_path);
        path.push("CommonModules");
        path.push(&full_module_name);
        path.push(&full_module_name);
        path.set_extension("mdo");

        let file = File::open(path).unwrap();
        let file = BufReader::new(file);
        from_reader(file).unwrap()
    }
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

        subsystem.child_systems = read_objects(&subsystem.subsystems, path_to_parent_subsystem.as_path());
        return subsystem;
    }
}

fn read_objects<T: MDObject>(object_names: &Vec<StringValue>, root_path: &Path) -> Vec<T> {
    object_names.iter()
        .map(|full_name| &full_name.value)
        .filter(|full_name| full_name.is_some())
        .map(|full_name| full_name.as_ref().unwrap())
        .map(|name| extract_name_from_full_name(name))
        .filter(|name| !name.is_empty())
        .map(|name| T::new(root_path, name))
        .collect()
}

fn extract_name_from_full_name(full_name: &String) -> String {
    return if let Some(split) = full_name.split_once(".") {
        split.1.to_string()
    } else {
        full_name.to_string()
    };
}



use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use quick_xml::de::from_reader;
use crate::general::GeneralInfo;
use crate::general::{BooleanValue, Item, MDObject};
use serde::{Deserialize, Serialize};

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
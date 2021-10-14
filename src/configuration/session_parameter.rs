use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use quick_xml::de::from_reader;
use crate::general::GeneralInfo;
use crate::general::{Item, MDObject, StringValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionParameter {
    #[serde(flatten)]
    pub general_info: GeneralInfo,
    pub synonym: Vec<Item>,
    #[serde(rename="type")]
    pub parameter_type: SessionParameterType,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionParameterType {
    pub types: Vec<StringValue>,
    pub string_qualifiers: ParameterQualifier,
    pub binary_qualifiers: ParameterQualifier,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterQualifier {
    #[serde(default)]
    pub length: i32,
}

impl MDObject for SessionParameter {
    fn new(root_path: &Path, subsystem_name: String) -> Self {
        let full_parameter_name = subsystem_name.to_owned();

        let mut path_to_parameter = PathBuf::new();
        path_to_parameter.push(root_path);
        path_to_parameter.push("SessionParameters");
        path_to_parameter.push(&full_parameter_name);
        path_to_parameter.push(&full_parameter_name);
        path_to_parameter.set_extension("mdo");

        let file = File::open(path_to_parameter).unwrap();
        let file = BufReader::new(file);
        let session_parameter: SessionParameter = from_reader(file).unwrap();

        return session_parameter;
    }
}

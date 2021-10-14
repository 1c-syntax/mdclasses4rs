use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use quick_xml::de::from_reader;
use crate::configuration::common_module::CommonModule;

use crate::configuration::configuration_root::ConfigurationRoot;
use crate::configuration::role::Role;
use crate::configuration::session_parameter::SessionParameter;
use crate::configuration::subsystem::Subsystem;
use crate::general;

pub mod configuration_root;
pub mod session_parameter;
pub mod common_module;
pub mod subsystem;
pub mod role;
pub mod language;

#[derive(Debug)]
pub struct Configuration {
    pub root: ConfigurationRoot,
    pub common_modules: Vec<CommonModule>,
    pub subsystems: Vec<Subsystem>,
    pub session_parameters: Vec<SessionParameter>,
    pub roles: Vec<Role>,
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
        let session_parameters_names = &root.session_parameters;
        let role_names = &root.roles;

        let common_modules = general::read_objects(common_modules_names, root_path);
        let subsystems = general::read_objects(subsystems_names, root_path);
        let session_parameters = general::read_objects(session_parameters_names, root_path);
        let roles = general::read_objects(role_names, root_path);

        Configuration {
            root,
            common_modules,
            subsystems,
            session_parameters,
            roles
        }
    }
}



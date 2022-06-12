use crate::settings::Instruction;
use crate::util::{get_path, listdir, get_file_name};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct App {
    pub instructions: HashMap<PathBuf, Instruction>,
    pub configs: HashMap<String, Vec<PathBuf>>,
}

impl App {
    pub fn new() -> Result<Self, String> {
        let mut instructions = HashMap::new();
        let mut configs = HashMap::new();

        for inst in listdir(get_path("CONMAN_INSTRUCTIONS")?)? {
            let i = inst.or(Err("Can't get dir entry (instructions dir)".to_string()))?;
            instructions.insert(i.path(), Instruction::from_file(&i.path())?);
        }

        for storage in listdir(get_path("CONMAN_CONFIGS")?)? {
            let storage = storage.or(Err("Can't get dir entry (config storage)".to_string()))?;
            if storage.path().is_dir() {
                let mut config_storage = Vec::new();
                for config in listdir(storage.path())? {
                    let config = config.or(Err("Can't get dir entry (config storage entry)"))?;
                    config_storage.push(config.path());
                }

                configs.insert(get_file_name(storage.path())?.0, config_storage);
            }
        }

        Ok(Self {
            instructions,
            configs,
        })
    }
}

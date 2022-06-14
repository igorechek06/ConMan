use crate::args::Args;
use crate::settings::Instruction;
use crate::util::{get_file_name, get_path, listdir};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct App {
    pub instructions: HashMap<PathBuf, Instruction>,
    pub configs: HashMap<String, Vec<PathBuf>>,
}

impl App {
    pub fn new(_args: &Args) -> Result<Self, String> {
        let mut instructions = HashMap::new();
        let mut configs = HashMap::new();

        // Parse instructions
        for inst in listdir(get_path("CONMAN_INSTRUCTIONS")?)? {
            let i = inst.or(Err("Can't get dir entry (instructions dir)".to_string()))?;
            instructions.insert(i.path(), Instruction::from_file(&i.path())?);
        }

        // Parse configs
        for conf_dir in listdir(get_path("CONMAN_CONFIGS")?)? {
            let conf_dir = conf_dir.or(Err("Can't get dir entry (config storage)".to_string()))?;

            if conf_dir.path().is_dir() {
                let mut config_storage = Vec::new();

                for conf in listdir(conf_dir.path())? {
                    let conf = conf.or(Err("Can't get dir entry (config storage entry)"))?;
                    if conf.path().is_file() {
                        config_storage.push(conf.path());
                    }
                }

                configs.insert(get_file_name(conf_dir.path())?.0, config_storage);
            }
        }

        Ok(Self {
            instructions,
            configs,
        })
    }

    pub fn list(&self, names: &Vec<String>) -> Result<HashMap<String, Vec<String>>, String> {
        let mut result = HashMap::new();
        for (inst_path, inst) in &self.instructions {
            let inst_name = &get_file_name(inst_path)?.0;
            let inst_name = inst.name.as_ref().unwrap_or(inst_name);

            // Filter isntructions
            if !names.is_empty() && !names.contains(inst_name) {
                continue;
            }

            // Find configs
            let mut configs = Vec::new();
            for (conf_name, conf_dir) in &self.configs {
                if conf_name == inst_name {
                    for conf in conf_dir {
                        configs.push(get_file_name(conf)?.0)
                    }
                    break;
                }
            }

            // Append results
            result.insert(inst_name.to_owned(), configs);
        }
        Ok(result)
    }
}

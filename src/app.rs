use crate::args::Args;
use crate::settings::Instruction;
use crate::util::path;
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

pub struct App {
    instructions: Vec<PathBuf>,
    configs: HashMap<PathBuf, Vec<PathBuf>>,
}

impl App {
    pub fn new(_args: &Args) -> Result<Self, String> {
        let mut instructions = Vec::new();
        let mut configs = HashMap::new();

        // Parse instructions
        for inst in path::list(path::get("CONMAN_INSTRUCTIONS")?)? {
            let inst = inst.or(Err("Can't get dir entry (instructions dir)".to_string()))?;
            instructions.push(inst.path());
        }

        // Parse configs
        for conf_dir in path::list(path::get("CONMAN_CONFIGS")?)? {
            let conf_dir = conf_dir.or(Err("Can't get dir entry (config storage)".to_string()))?;

            if conf_dir.path().is_dir() {
                let mut confs = Vec::new();

                for conf in path::list(conf_dir.path())? {
                    let conf = conf.or(Err("Can't get dir entry (config storage entry)"))?;
                    if conf.path().is_file() {
                        confs.push(conf.path());
                    }
                }

                configs.insert(conf_dir.path(), confs);
            }
        }

        Ok(Self {
            instructions,
            configs,
        })
    }

    pub fn list(&self, names: &Vec<String>) -> Result<BTreeMap<String, Vec<String>>, String> {
        let mut result = BTreeMap::new();
        for inst_path in &self.instructions {
            let inst_name = path::name(inst_path)?.0;

            // Filter isntructions
            if !names.is_empty() && !names.contains(&inst_name) {
                continue;
            }

            // Find configs
            let mut configs = Vec::new();
            for (conf_path, conf_dir) in &self.configs {
                let conf_name = path::name(conf_path)?.0;
                if conf_name == inst_name {
                    for conf in conf_dir {
                        configs.push(path::name(conf)?.0)
                    }
                    break;
                }
            }

            // Append results
            result.insert(inst_name.to_owned(), configs);
        }
        Ok(result)
    }

    pub fn contains(&self, name: &String) -> Result<bool, String> {
        let mut inst = false;
        let mut conf = false;

        for inst_path in &self.instructions {
            if path::name(inst_path)?.0 == *name {
                inst = true;
            }
        }

        for (conf_path, _) in &self.configs {
            if path::name(conf_path)?.0 == *name {
                conf = true;
            }
        }

        Ok(inst && conf)
    }

    pub fn get(
        &self,
        name: &String,
    ) -> Result<((PathBuf, Instruction), (PathBuf, Vec<PathBuf>)), String> {
        let mut inst = Err(format!("Can't find instruction ({})", name));
        let mut conf = Err(format!("Cat't find config ({})", name));

        for inst_path in &self.instructions {
            if path::name(inst_path)?.0 == *name {
                inst = Ok((inst_path.to_owned(), Instruction::from_file(inst_path)?));
            }
        }

        for (conf_path, confs) in &self.configs {
            if path::name(conf_path)?.0 == *name {
                conf = Ok((conf_path.to_owned(), confs.to_owned()));
            }
        }

        return Ok((inst?, conf?));
    }
}

use crate::util::path;
use std::collections::BTreeMap;
use std::path::PathBuf;

pub struct App {
    pub instructions: BTreeMap<String, PathBuf>,
    pub configs: BTreeMap<String, (PathBuf, BTreeMap<String, PathBuf>)>,
}

impl App {
    pub fn new() -> Result<Self, String> {
        let mut instructions = BTreeMap::new();
        let mut configs = BTreeMap::new();

        // Parse instructions
        for inst in path::list(path::config_dir()?.0)? {
            let inst = inst.or(Err("Can't get dir entry (instructions dir)".to_string()))?;
            instructions.insert(path::name(inst.path())?.0, inst.path());
        }

        // Parse configs
        for conf_dir in path::list(path::config_dir()?.1)? {
            let conf_dir = conf_dir.or(Err("Can't get dir entry (config storage)".to_string()))?;
            let conf_name = path::name(conf_dir.path())?.0;

            if conf_dir.path().is_dir() && instructions.contains_key(&conf_name) {
                let mut confs = BTreeMap::new();

                for conf in path::list(conf_dir.path())? {
                    let conf = conf.or(Err("Can't get dir entry (config storage entry)"))?;
                    if conf.path().is_file() {
                        confs.insert(path::name(conf.path())?.0, conf.path());
                    }
                }

                configs.insert(conf_name, (conf_dir.path(), confs));
            }
        }

        // Fix problems
        if instructions.len() != configs.len() {
            for (inst_name, _) in instructions.clone() {
                if !configs.contains_key(&inst_name) {
                    instructions.remove(&inst_name);
                }
            }
        }

        Ok(Self {
            instructions,
            configs,
        })
    }

    pub fn contains(&self, name: &str) -> bool {
        self.instructions.contains_key(name) && self.configs.contains_key(name)
    }
}

use crate::settings::Instruction;
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
            instructions.insert(path::name(inst.path())?.0, inst.path());
        }

        // Parse configs
        for conf_dir in path::list(path::config_dir()?.1)? {
            let conf_name = path::name(conf_dir.path())?.0;

            if conf_dir.path().is_dir() && instructions.contains_key(&conf_name) {
                let mut confs = BTreeMap::new();

                for conf in path::list(conf_dir.path())? {
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

    pub fn exist(&self, name: &str) -> bool {
        self.instructions.contains_key(name) && self.configs.contains_key(name)
    }

    pub fn instruction(&self, name: &str) -> Result<&PathBuf, String> {
        self.instructions
            .get(name)
            .ok_or(format!("Instruction does not exist ({})", name))
    }

    pub fn parse_instruction(&self, name: &str) -> Result<(Instruction, &PathBuf), String> {
        let path = self.instruction(name)?;
        Ok((Instruction::from_file(path, self)?, path))
    }

    pub fn configs(&self, name: &str) -> Result<&(PathBuf, BTreeMap<String, PathBuf>), String> {
        self.configs
            .get(name)
            .ok_or(format!("Config does not exist ({})", name))
    }

    pub fn config(&self, name: &str, number: &usize) -> Result<&PathBuf, String> {
        let configs = &self.configs(&name)?.1;
        let keys: Vec<String> = configs.keys().cloned().collect();
        let key = keys
            .get(number - 1)
            .ok_or(format!("Config does not exist ({})", number))?;

        Ok(configs.get(key).unwrap())
    }

    pub fn add(&mut self, name: &str) -> Result<(), String> {
        if self.exist(name) {
            return Err(format!("Instruction already exist ({})", name));
        }

        let (mut inst_path, mut conf_path) = path::config_dir()?;
        inst_path.push(format!("{}.yml", name));
        conf_path.push(name);

        path::mkfile(&inst_path)?;
        path::mkdir(&conf_path)?;

        self.instructions.insert(name.to_string(), inst_path);
        self.configs
            .insert(name.to_string(), (conf_path, BTreeMap::new()));

        Ok(())
    }

    pub fn del(&mut self, name: &str, numbers: &Vec<usize>) -> Result<(), String> {
        if !numbers.is_empty() {
            let configs = &mut self
                .configs
                .get_mut(name)
                .ok_or(format!("Config does not exist ({})", name))?
                .1;
            let keys: Vec<String> = configs.keys().cloned().collect();

            for number in numbers {
                let key = keys
                    .get(number - 1)
                    .ok_or(format!("Config does not exist ({})", number))?;

                path::rm(configs.get(key).unwrap())?;
                configs.remove(key);
            }
        } else {
            let inst = self.instruction(name)?;
            let confs = self.configs(name)?;

            path::rm(inst)?;
            path::rm(&confs.0)?;

            self.instructions.remove(name);
            self.configs.remove(name);
        }
        Ok(())
    }
}

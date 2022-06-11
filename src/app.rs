use crate::settings::Instruction;
use crate::util::get_path;
use std::collections::HashMap;
use std::fs::read_dir;

pub struct App {
    pub instructions: HashMap<String, Instruction>,
    pub configs: HashMap<String, Instruction>,
}

impl App {
    pub fn new() -> Result<Self, String> {
        let mut instructions = HashMap::new();
        let mut configs = HashMap::new();
// TODO
        return Ok(Self {
            instructions,
            configs,
        });
    }
}

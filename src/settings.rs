use serde::{Deserialize, Serialize};
use serde_yaml::{from_reader, to_writer};
use std::{collections::HashMap, fs::File, path::Path};

type Objects = HashMap<String, Vec<String>>;

// Instructions
#[derive(Serialize, Deserialize)]
pub struct Instruction {
    // Required
    pub objects: Objects,
    // Optional
    pub name: Option<String>,
    pub paths: Option<HashMap<String, String>>,
    pub secrets: Option<Objects>,
}

impl Instruction {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = File::open(&path).or(Err(format!(
            "Can't open file ({})",
            path.as_ref().display()
        )))?;

        Ok(from_reader(file).unwrap_or_default())
    }

    pub fn _to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let file = File::create(&path).or(Err(format!(
            "Can't create file ({})",
            path.as_ref().display()
        )))?;
        let result = to_writer(file, self).or(Err(format!(
            "Can't save file ({})",
            path.as_ref().display()
        )))?;

        Ok(result)
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            objects: HashMap::new(),
            name: None,
            paths: None,
            secrets: None,
        }
    }
}

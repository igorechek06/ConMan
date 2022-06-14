use serde::{Deserialize, Serialize};
use serde_yaml::{from_reader, to_writer};
use std::{collections::HashMap, fs::File, path::Path};

// Instructions
#[derive(Serialize, Deserialize)]
pub struct Instruction {
    pub name: Option<String>,
    pub paths: Option<HashMap<String, String>>,
    pub objects: HashMap<String, Vec<String>>,
    pub secrets: Option<HashMap<String, Vec<String>>>,
}

impl Instruction {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = File::open(&path).or(Err(format!(
            "Can't open the file ({})",
            path.as_ref().display()
        )))?;

        let result: Self = from_reader(file).or(Err(format!(
            "Can't parse the file ({})",
            path.as_ref().display()
        )))?;

        Ok(result)
    }

    pub fn _to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let file = File::create(&path).or(Err(format!(
            "Can't create the file ({})",
            path.as_ref().display()
        )))?;
        let result = to_writer(file, self).or(Err(format!(
            "Can't save the file ({})",
            path.as_ref().display()
        )))?;

        Ok(result)
    }
}

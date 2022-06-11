use std::{collections::HashMap, fs::File, path::Path};
use serde::{Deserialize, Serialize};
use serde_yaml::{from_reader, to_writer};

// Instructions
#[derive(Serialize, Deserialize)]
pub struct Instruction {
    pub name: Option<String>,
    pub paths: HashMap<String, String>,
    pub objects: Vec<String>,
    pub secrets: Vec<String>,
}

impl Instruction {
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let file =
            File::open(&path).or(Err(format!("Unable to open file ({})", path.display())))?;
        return Ok(from_reader(file).or(Err(format!("Unable to parse file ({})", path.display())))?);
    }

    pub fn to_file(&self, path: &Path) -> Result<(), String> {
        let file =
            File::create(&path).or(Err(format!("Unable to create file ({})", path.display())))?;
        return Ok(
            to_writer(file, self).or(Err(format!("Unable to save file ({})", path.display())))?
        );
    }
}

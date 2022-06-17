use serde::{Deserialize, Serialize};
use serde_yaml::{from_reader, to_writer};
use std::{collections::HashMap, fs::File, path::Path};

type Objects = HashMap<String, Vec<String>>;

// Instructions
#[derive(Serialize, Deserialize)]
pub struct Instruction {
    // Optional
    pub include: Option<Vec<String>>,
    pub paths: Option<HashMap<String, String>>,
    pub objects: Option<Objects>,
    pub secrets: Option<Objects>,
}

impl Instruction {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = File::open(&path).or(Err(format!(
            "Can't open file ({})",
            path.as_ref().display()
        )))?;

        from_reader(file).map_err(|err| {
            format!(
                "Can't parse instruction ({}) :: {}",
                path.as_ref().display(),
                err.to_string()
            )
        })
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

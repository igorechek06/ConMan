use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::app::App;
use crate::util::path;

// Raw instruction
#[derive(Serialize, Deserialize)]
struct RawInstruction {
    // Optional
    include: Option<Vec<String>>,
    paths: Option<HashMap<String, String>>,
    objects: Option<HashMap<String, Vec<String>>>,
    secrets: Option<HashMap<String, Vec<String>>>,
}

// Instruction
pub struct Instruction {
    pub paths: HashMap<String, PathBuf>,
    pub objects: HashMap<PathBuf, Vec<PathBuf>>,
    pub secrets: HashMap<PathBuf, Vec<PathBuf>>,
}

impl Instruction {
    pub fn from_file<P: AsRef<Path>>(path: P, app: &App) -> Result<Self, String> {
        let path = path.as_ref();
        let file = File::open(&path).or(Err(format!("Can't open file ({})", path.display())))?;

        let raw: RawInstruction = from_reader(file).map_err(|err| {
            format!(
                "Can't parse instruction ({}) :: {}",
                path.display(),
                err.to_string()
            )
        })?;

        let include = include(raw.include)?;

        Ok(Self {
            paths: paths(raw.paths, &include)?,
            objects: objects(raw.objects, &include)?,
            secrets: objects(raw.secrets, &include)?,
        })
    }
}

fn parse_path(path: String) -> Result<PathBuf, String> {
    todo!()
}

fn include(raw: Option<Vec<String>>) -> Result<Vec<Instruction>, String> {
    todo!()
}

fn paths(
    raw: Option<HashMap<String, String>>,
    include: &Vec<Instruction>,
) -> Result<HashMap<String, PathBuf>, String> {
    todo!()
}

fn objects(
    raw: Option<HashMap<String, Vec<String>>>,
    include: &Vec<Instruction>,
) -> Result<HashMap<PathBuf, Vec<PathBuf>>, String> {
    todo!()
}

use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

use crate::{app::App, util::path};

type Objects = HashMap<String, Vec<String>>;

// Instructions
#[derive(Serialize, Deserialize)]
pub struct Instruction {
    // Optional
    include: Option<Vec<String>>,
    paths: Option<HashMap<String, String>>,
    objects: Option<Objects>,
    secrets: Option<Objects>,
}

impl Instruction {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = File::open(&path).or(Err(format!(
            "Can't open file ({})",
            path.as_ref().display()
        )))?;

        let result: Self = from_reader(file).map_err(|err| {
            format!(
                "Can't parse instruction ({}) :: {}",
                path.as_ref().display(),
                err.to_string()
            )
        })?;

        if let Some(include) = &result.include {
            let name = path::name(path)?.0;
            if include.contains(&name) {
                return Err(format!("Recursive include ({})", name));
            }
        }

        Ok(result)
    }

    pub fn include(&self, app: &App) -> Result<Vec<Self>, String> {
        let mut result = Vec::new();

        if let Some(include) = &self.include {
            for name in include {
                result.push(app.parse(name)?)
            }
        }

        Ok(result)
    }

    pub fn paths(&self, app: &App) -> Result<HashMap<String, PathBuf>, String> {
        let mut result = HashMap::new();

        if let Some(paths) = &self.paths {
            for (name, path) in paths {
                result.insert(name.to_owned(), Path::new(path).to_path_buf());
            }
        }
        for inst in self.include(app)? {
            result.extend(inst.paths(app)?)
        }

        Ok(result)
    }

    pub fn objects(&self, app: &App) -> Result<HashMap<PathBuf, Vec<PathBuf>>, String> {
        let mut result = HashMap::new();

        if let Some(objects) = &self.objects {
            for (root, paths) in objects {
                result.insert(
                    Path::new(root).to_path_buf(),
                    paths
                        .to_owned()
                        .iter()
                        .map(|p| Path::new(p).to_path_buf())
                        .collect::<Vec<PathBuf>>(),
                );
            }
        }
        for inst in self.include(app)? {
            result.extend(inst.objects(app)?)
        }

        Ok(result)
    }

    pub fn secrets(&self, app: &App) -> Result<HashMap<PathBuf, Vec<PathBuf>>, String> {
        let mut result = HashMap::new();

        if let Some(secrets) = &self.secrets {
            for (root, paths) in secrets {
                result.insert(
                    Path::new(root).to_path_buf(),
                    paths
                        .to_owned()
                        .iter()
                        .map(|p| Path::new(p).to_path_buf())
                        .collect::<Vec<PathBuf>>(),
                );
            }
        }
        for inst in self.include(app)? {
            result.extend(inst.secrets(app)?)
        }

        Ok(result)
    }
}

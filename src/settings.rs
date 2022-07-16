use crate::app::App;
use crate::util::{path, str_err};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::collections::HashMap;
use std::env::consts;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct RawEntries {
    path: String,
    entries: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RawInstruction {
    include: Option<Vec<String>>,
    save: Option<HashMap<String, RawEntries>>,
    os: Vec<String>,
}

impl RawInstruction {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref();
        let file = str_err(File::open(&path))?;

        from_reader(file).map_err(|err| {
            format!(
                "Can't parse instruction ({}) :: {}",
                path.display(),
                err.to_string()
            )
        })
    }
}

#[derive(Clone)]
pub struct Entries {
    pub root: PathBuf,
    pub add: Vec<PathBuf>,
    pub del: Vec<PathBuf>,
}

pub struct Instruction {
    pub save: HashMap<String, Entries>,
}

impl Instruction {
    pub fn from_file<P: AsRef<Path>>(path: P, app: &App) -> Result<Self, String> {
        Self::from_raw(RawInstruction::from_file(path)?, app)
    }

    pub fn from_raw(raw: RawInstruction, app: &App) -> Result<Self, String> {
        let os: Vec<String> = raw.os.iter().map(|o| o.to_lowercase()).collect();
        if !os.contains(&consts::OS.to_string()) {
            return Err(format!(
                "Instruction only support {} not {}",
                os.join(", "),
                consts::OS
            ));
        }

        Ok(Self {
            save: parse_save(
                raw.save,
                parse_include(raw.include, app)?
                    .iter()
                    .map(|i| i.save.clone())
                    .collect(),
            )?,
        })
    }
}

fn parse_path(path: String) -> Result<PathBuf, String> {
    let mut path = path;
    let regex = Regex::new(r#"\{(?P<name>.+)\}"#).unwrap();

    for m in regex.captures_iter(&path.clone()) {
        let name = m.name("name").unwrap().as_str().trim();

        let old = format!("{{{}}}", name);
        let new = path::get(name)?.display().to_string();

        path = path.replace(&old, &new);
    }

    Ok(PathBuf::from(&path))
}

fn parse_entries(raw: RawEntries) -> Result<Entries, String> {
    let root = parse_path(raw.path)?;
    let mut add = Vec::new();
    let mut del = Vec::new();

    for e in raw.entries {
        let mut path = root.clone();
        if e.starts_with("-") {
            path.push(e.strip_prefix("-").unwrap());
            if !del.contains(&path) {
                del.push(path);
            }
        } else {
            path.push(e.strip_prefix("+").unwrap_or(&e));
            if !add.contains(&path) {
                add.push(path);
            }
        }
    }

    Ok(Entries { root, add, del })
}

fn parse_include(raw: Option<Vec<String>>, app: &App) -> Result<Vec<Instruction>, String> {
    fn include_raw(
        raw: Option<Vec<String>>,
        included: Vec<String>,
        app: &App,
    ) -> Result<Vec<(String, RawInstruction)>, String> {
        let mut result = Vec::new();
        let mut included = included;

        if let Some(raw) = raw {
            for name in &raw {
                if included.contains(name) {
                    return Err(format!("Recursive inclusion ({})", name));
                }

                included.push(name.clone());

                let inst = RawInstruction::from_file(app.instruction(&name)?)?;

                result.extend(include_raw(inst.include.clone(), included.clone(), app)?);
                result.push((name.to_string(), inst))
            }
        }
        Ok(result)
    }

    let mut result = Vec::new();
    for (name, raw) in include_raw(raw, Vec::new(), app)? {
        result.push(
            Instruction::from_raw(raw, app)
                .map_err(|e| format!("An error occurred while including ({}) :: {}", name, e))?,
        )
    }
    Ok(result)
}

fn parse_save(
    raw: Option<HashMap<String, RawEntries>>,
    extend: Vec<HashMap<String, Entries>>,
) -> Result<HashMap<String, Entries>, String> {
    let mut result = HashMap::new();

    if let Some(raw) = raw {
        for (name, entries) in raw {
            result.insert(name, parse_entries(entries)?);
        }
    }
    for e in extend {
        result.extend(e);
    }

    Ok(result)
}

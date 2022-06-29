use crate::app::App;
use crate::util::path;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

// Raw instruction
#[derive(Serialize, Deserialize)]
pub struct RawInstruction {
    // Optional
    include: Option<Vec<String>>,
    objects: Option<HashMap<String, Vec<String>>>,
}

impl RawInstruction {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref();
        let file = File::open(&path).or(Err(format!("Can't open file ({})", path.display())))?;

        from_reader(file).map_err(|err| {
            format!(
                "Can't parse instruction ({}) :: {}",
                path.display(),
                err.to_string()
            )
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Entry {
    Add { path: PathBuf },
    Del { path: PathBuf },
}

// Instruction
pub struct Instruction {
    pub objects: Vec<Entry>,
}

impl Instruction {
    pub fn from_file<P: AsRef<Path>>(path: P, app: &App) -> Result<Self, String> {
        Self::from_raw(RawInstruction::from_file(path)?, app)
    }

    pub fn from_raw(raw: RawInstruction, app: &App) -> Result<Self, String> {
        Ok(Self {
            objects: objects(
                raw.objects,
                include(raw.include, app)?
                    .iter()
                    .map(|i| i.objects.clone())
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

    Ok(Path::new(&path).to_path_buf())
}

fn parse_entry(root: &PathBuf, entry: &String) -> Entry {
    let mut path = root.clone();
    if entry.starts_with("-") {
        path.push(entry.strip_prefix("-").unwrap());
        return Entry::Del { path };
    } else {
        path.push(entry.strip_prefix("+").unwrap_or(entry));
        return Entry::Add { path };
    }
}

fn include(raw: Option<Vec<String>>, app: &App) -> Result<Vec<Instruction>, String> {
    fn include_raw(
        raw: Option<Vec<String>>,
        included: Vec<String>,
        app: &App,
    ) -> Result<Vec<RawInstruction>, String> {
        let mut result = Vec::new();
        let mut included = included;

        if let Some(raw) = raw {
            for name in &raw {
                if included.contains(name) {
                    return Err(format!("Recursive inclusion ({})", name));
                }
                included.push(name.clone());

                let inst = RawInstruction::from_file(
                    app.instructions
                        .get(name)
                        .ok_or(format!("Instruction does not exist ({})", name))?,
                )?;

                result.extend(include_raw(inst.include.clone(), included.clone(), app)?);
                result.push(inst)
            }
        }
        Ok(result)
    }

    let mut result = Vec::new();
    for raw in include_raw(raw, Vec::new(), app)? {
        result.push(Instruction::from_raw(raw, app)?)
    }
    Ok(result)
}

fn objects(
    raw: Option<HashMap<String, Vec<String>>>,
    extend: Vec<Vec<Entry>>,
) -> Result<Vec<Entry>, String> {
    let mut result = Vec::new();

    if let Some(raw) = raw {
        for (root, raw_entries) in &raw {
            for entry in raw_entries {
                let entry = parse_entry(&parse_path(root.clone())?, &entry);
                if !result.contains(&entry) {
                    result.push(entry);
                }
            }
        }
    }
    for e in extend {
        result.extend(e);
    }

    Ok(result)
}

use std::fs::{create_dir_all, read_dir, remove_dir_all, remove_file, ReadDir};
use std::path::{Path, PathBuf};

use regex::Regex;

pub fn get_path(path_type: &str) -> Result<PathBuf, String> {
    let mut path = match path_type {
        "HOME" => dirs::home_dir(),
        "DATA" => dirs::data_dir(),
        "CONFIG" | "CONMAN" | "CONMAN_CONFIGS" | "CONMAN_INSTRUCTIONS" => dirs::config_dir(),
        _ => return Err(format!("Unknown path type ({})", path_type)),
    }
    .ok_or(format!("Unable to get dir ({})", path_type))?;

    match path_type {
        "CONMAN" => path.extend(["conman"]),
        "CONMAN_CONFIGS" => path.extend(["conman", "configs"]),
        "CONMAN_INSTRUCTIONS" => path.extend(["conman", "instructions"]),
        _ => {}
    }

    if path.exists() && !path.is_dir() {
        remove_file(&path).or(Err(format!("Can't remove file ({})", path.display())))?;
    }
    if !path.exists() {
        create_dir_all(&path).or(Err(format!("Can't create dir ({})", path.display())))?;
    }

    Ok(path)
}

pub fn listdir<P: AsRef<Path>>(path: P) -> Result<ReadDir, String> {
    return read_dir(&path).or(Err(format!("Can't read dir ({})", path.as_ref().display())));
}

pub fn get_file_name<P: AsRef<Path>>(path: P) -> Result<(String, Option<String>), String> {
    let regex = Regex::new(r#"^(?P<name>\.?.*?)(\.(?P<ext>[^.]+))?$"#).unwrap();
    let name = path
        .as_ref()
        .file_name()
        .ok_or("Can't get file name :: Path is empty")?
        .to_str()
        .ok_or("Can't convert file name to UTF-8 string")?
        .to_string();

    let result = regex.captures(&name).ok_or("Incorrect file name")?;
    return Ok((
        result
            .name("name")
            .ok_or("Incorrect file name")?
            .as_str()
            .to_string(),
        result.name("ext").map(|m| m.as_str().to_string()),
    ));
}

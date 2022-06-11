use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;

pub fn get_path(path_type: &str) -> Result<PathBuf, String> {
    let mut path = match path_type {
        "HOME" => dirs::home_dir(),
        "DATA" => dirs::data_dir(),
        "CONFIG" | "CONMAN" => dirs::config_dir(),
        _ => return Err(format!("Unknown path type ({})", path_type)),
    }
    .ok_or(format!("Unable to get dir ({})", path_type))?;

    match path_type {
        "CONMAN" => path.extend(["conman"]),
        "CONMAN_INSTRUCTIONS" => path.extend(["conman", "instructions"]),
        "CONMAN_CONFIGS" => path.extend(["conman", "configs"]),
        _ => {}
    }

    if path.is_dir() {
        remove_dir_all(&path).or(Err(format!("Can't remove dir {}", path.display())))?;
    }
    if !path.exists() {
        create_dir_all(&path).or(Err(format!("Can't create dir ({})", path.display())))?;
    }

    Ok(path)
}

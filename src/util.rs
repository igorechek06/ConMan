use std::path::PathBuf;

fn get_dir(name: String) -> Result<PathBuf, String> {
    let path = match name.as_str() {
        "HOME" => dirs::home_dir(),
        "DATA" => dirs::data_dir(),
        "CONFIG" => dirs::config_dir(),
        _ => {
            return Err(format!("Error :: Unknown path type `{}`", name));
        }
    };

    if let Some(path) = path {
        return Ok(path);
    } else {
        return Err(format!("Error :: {} path can't be found", name));
    }
}

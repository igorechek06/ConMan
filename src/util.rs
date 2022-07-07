use std::fmt::Display;

pub fn print_err<R, E: Display>(result: Result<R, E>) -> Result<R, E> {
    if let Err(err) = &result {
        eprintln!("Error :: {}", err);
    }
    result
}

pub fn str_err<R, E: Display>(result: Result<R, E>) -> Result<R, String> {
    return result.map_err(|e| e.to_string());
}

pub mod path {
    use super::*;
    use regex::Regex;
    use std::env::temp_dir;
    use std::fs::{copy, create_dir_all, remove_dir_all, remove_file, File, ReadDir};
    use std::path::{Path, PathBuf};
    use uuid::Uuid;

    pub fn mkdir<P: AsRef<Path>>(path: P) -> Result<(), String> {
        let path = path.as_ref();

        if path.exists() && !path.is_dir() {
            rm(path)?;
        }
        if !path.exists() {
            str_err(create_dir_all(path))?;
        }

        Ok(())
    }

    pub fn mkfile<P: AsRef<Path>>(path: P) -> Result<(), String> {
        let path = path.as_ref();

        if path.exists() && !path.is_file() {
            rm(path)?;
        }
        if !path.exists() {
            str_err(File::create(path))?;
        }

        Ok(())
    }

    pub fn rm<P: AsRef<Path>>(path: P) -> Result<(), String> {
        let path = path.as_ref();

        if path.is_dir() {
            str_err(remove_dir_all(path))?;
        } else {
            str_err(remove_file(path))?;
        }

        Ok(())
    }

    pub fn cp<F: AsRef<Path>, T: AsRef<Path>>(from: F, to: T) -> Result<(), String> {
        let from = from.as_ref();
        let to = to.as_ref();

        if from.is_dir() {
            let to = add(to, name(from)?.2);
            mkdir(&to)?;

            for item in list(from)? {
                let item = name(str_err(item)?.path())?.2;
                cp(add(from, &item), add(&to, &item))?;
            }
        } else if from.is_file() {
            str_err(copy(from, to))?;
        }

        Ok(())
    }

    pub fn list<P: AsRef<Path>>(path: P) -> Result<ReadDir, String> {
        str_err(path.as_ref().read_dir())
    }

    pub fn name<P: AsRef<Path>>(path: P) -> Result<(String, Option<String>, String), String> {
        let regex = Regex::new(r#"^(?P<name>\.?.*?)(\.(?P<ext>[^.]+))?$"#).unwrap();
        let name = path
            .as_ref()
            .file_name()
            .ok_or("Can't get file name :: Path is empty")?
            .to_str()
            .ok_or("Can't convert file name to UTF-8 string")?
            .to_string();

        let result = regex.captures(&name).ok_or("Incorrect file name")?;

        Ok((
            result
                .name("name")
                .ok_or("Incorrect file name")?
                .as_str()
                .to_string(),
            result.name("ext").map(|m| m.as_str().to_string()),
            result.get(0).unwrap().as_str().to_string(),
        ))
    }

    pub fn add<P: AsRef<Path>, A: AsRef<Path>>(path: P, add: A) -> PathBuf {
        let mut path = path.as_ref().to_path_buf();
        path.push(add);
        path
    }

    pub fn get(path_type: &str) -> Result<PathBuf, String> {
        let path = match path_type {
            "HOME" => dirs::home_dir(),
            "DATA" => dirs::data_dir(),
            "LOCAL" => dirs::data_local_dir(),
            "CONFIG" => dirs::config_dir(),
            "PREFERENCE" => dirs::preference_dir(),
            _ => return Err(format!("Unknown path type ({})", path_type)),
        }
        .ok_or(format!("Unable to get dir ({})", path_type))?;

        mkdir(&path)?;

        Ok(path)
    }

    pub fn config_dir() -> Result<(PathBuf, PathBuf), String> {
        let mut config = dirs::config_dir().expect("Unable to get config dir");
        let mut data = dirs::data_local_dir().expect("Unable to get data dir");

        config.push("conman");
        data.push("conman");

        mkdir(&config)?;
        mkdir(&data)?;

        Ok((config, data))
    }

    pub fn tmp_dir() -> Result<PathBuf, String> {
        let mut path = temp_dir();
        path.push(Uuid::new_v4().to_string());
        mkdir(&path)?;
        Ok(path)
    }
}

// pub mod archive {
//     use std::fmt::Display;
//     use std::path::Path;
//     use std::process::Command;
//
//     fn repr<S: Display>(text: S) -> String {
//         format!(r#"{}"#, text.to_string().escape_default())
//     }
//
//     pub fn zip<P>(
//         archive: P,
//         inpath: P,
//         compression: &u8,
//         password: Option<&String>,
//     ) -> Result<(), String>
//     where
//         P: AsRef<Path>,
//     {
//         let archive = repr(archive.as_ref().display());
//         let entry = repr(inpath.as_ref().display());
//         let compression = format!("-mx{}", compression);
//         let password = password.map_or("".to_string(), |p| format!("-P{}", repr(p)));
//
//         let cmd = Command::new("7z")
//             .arg("a")
//             .arg("-y")
//             .arg(compression)
//             .arg(password)
//             .arg(archive)
//             .arg(entry)
//             .output()
//             .or(Err("Process failed to execute"))?;
//
//         if !cmd.status.success() {
//             eprintln!("{}", String::from_utf8(cmd.stderr).unwrap().trim());
//         }
//
//         Ok(())
//     }
//
//     pub fn unzip<P: AsRef<Path>>(archive: P, outpath: P) -> Result<(), String> {
//         todo!()
//     }
// }

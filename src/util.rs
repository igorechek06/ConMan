use std::fmt::Display;

pub fn err<E: Display>(result: Result<(), E>) -> i32 {
    match result {
        Ok(..) => 0,
        Err(error) => {
            eprintln!("Error :: {}", error);
            1
        }
    }
}

pub mod path {
    use regex::Regex;
    use std::fs::{create_dir_all, remove_dir_all, remove_file, File, ReadDir};
    use std::path::{Path, PathBuf};

    pub fn mkdir<P: AsRef<Path>>(path: P) -> Result<(), String> {
        let path = path.as_ref();

        if path.exists() && !path.is_dir() {
            rm(&path)?;
        }
        if !path.exists() {
            create_dir_all(&path).or(Err(format!("Can't create dir ({})", path.display())))?;
        }

        Ok(())
    }

    pub fn mkfile<P: AsRef<Path>>(path: P) -> Result<(), String> {
        let path = path.as_ref();

        if path.exists() && !path.is_file() {
            rm(&path)?;
        }
        if !path.exists() {
            File::create(&path).or(Err(format!("Can't create file ({})", path.display())))?;
        }

        Ok(())
    }

    pub fn rm<P: AsRef<Path>>(path: P) -> Result<(), String> {
        let path = path.as_ref();

        if path.is_dir() {
            remove_dir_all(&path).or(Err(format!("Can't remove dir ({})", path.display())))?
        } else {
            remove_file(&path).or(Err(format!("Can't remove file ({})", path.display())))?;
        }

        Ok(())
    }

    pub fn list<P: AsRef<Path>>(path: P) -> Result<ReadDir, String> {
        path.as_ref()
            .read_dir()
            .or(Err(format!("Can't read dir ({})", path.as_ref().display())))
    }

    pub fn name<P: AsRef<Path>>(path: P) -> Result<(String, Option<String>), String> {
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
        ))
    }

    pub fn get(path_type: &str) -> Result<PathBuf, String> {
        let path = match path_type {
            "HOME" => dirs::home_dir(),
            "DATA" => dirs::data_dir(),
            "LOCAL" => dirs::data_local_dir(),
            "CONFIGS" => dirs::config_dir(),
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
}

pub mod archive {
    use std::fmt::Display;
    use std::path::Path;
    use std::process::Command;

    fn repr<S: Display>(text: S) -> String {
        format!(r#"{}"#, text.to_string().escape_default())
    }

    pub fn zip<P>(
        archive: P,
        include: &Vec<P>,
        exclude: &Vec<P>,
        compression: &u8,
        password: Option<&String>,
    ) -> Result<(), String>
    where
        P: AsRef<Path>,
    {
        let archive = repr(archive.as_ref().display());
        let include: Vec<String> = include.iter().map(|p| repr(p.as_ref().display())).collect();
        let exclude: Vec<String> = exclude
            .iter()
            .map(|p| format!("-xr!{}", repr(p.as_ref().display())))
            .collect();
        let compression = format!("-mx{}", compression);
        let password = password.map_or("".to_string(), |p| format!("-P{}", repr(p)));

        let cmd = Command::new("7z")
            .arg("a")
            .arg("-y")
            .arg("-spf")
            .arg(compression)
            .arg(password)
            .arg(archive)
            .args(include)
            .args(exclude)
            .output()
            .or(Err("Process failed to execute"))?;

        if !cmd.status.success() {
            eprintln!("{}", String::from_utf8(cmd.stderr).unwrap().trim());
        }

        Ok(())
    }

    // pub fn unzip<P: AsRef<Path>>(archive: P, outdir: P) -> Result<(), String> {
    //     todo!()
    // }
}

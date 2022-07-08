extern crate chrono;
extern crate clap;
extern crate dirs;
extern crate open;
extern crate regex;
extern crate rpassword;
extern crate serde;
extern crate serde_yaml;
extern crate uuid;

mod app;
mod args;
mod settings;
mod util;

use app::App;
use args::{Action, Args};
use chrono::Local;
use clap::Parser;
use std::path::PathBuf;
use util::{archive, path};

/*
TODO: Add color output
TODO: Add i18n
*/

pub fn run() -> i32 {
    let args = Args::parse();
    let result = match &args.action {
        Action::Help => Ok(()),
        Action::List { names } => run_list(names),
        Action::Add { names } => run_add(names),
        Action::Del { names, number } => run_del(names, number),
        Action::Edit { name } => run_edit(name),
        Action::Save {
            name,
            path,
            compression,
            password,
        } => run_save(name, path, compression, password),
        Action::Load { file, name } => run_load(file, name),
        Action::Use { name, number } => run_use(name, number),
    };

    if let Err(err) = &result {
        eprintln!("Error :: {}", err);
    }
    result.is_err() as i32
}

fn run_list(names: &Vec<String>) -> Result<(), String> {
    let app = App::new()?;
    let mut result = String::new();

    if app.instructions.is_empty() {
        return Err(format!("Instructions not found"));
    }

    for name in names {
        if !app.exist(name) {
            return Err(format!("Instruction does not exist ({})", name));
        }
    }

    for (name, _) in &app.instructions {
        if names.is_empty() {
            result += format!("{} ({})\n", name, app.configs[name].1.len()).as_str();
        } else if names.contains(name) {
            result += format!("\n{}\n", name).as_str();
            for (num, conf_name) in app.configs[name].1.keys().enumerate() {
                result += format!("  {} - {}\n", num + 1, conf_name).as_str();
            }
        }
    }

    println!("{}", result.trim());

    Ok(())
}

fn run_add(names: &Vec<String>) -> Result<(), String> {
    let mut app = App::new()?;
    for name in names {
        app.add(name)?;
    }
    Ok(())
}

fn run_del(names: &Vec<String>, numbers: &Vec<usize>) -> Result<(), String> {
    let mut app = App::new()?;
    for name in names {
        app.del(name, numbers)?;
    }
    Ok(())
}

fn run_edit(name: &String) -> Result<(), String> {
    open::that(App::new()?.instruction(&name)?)
        .or(Err(format!("Can't open file in system editor ({})", name)))
}

fn run_save(
    name: &String,
    path: &Option<String>,
    compression: &u8,
    password: &Option<String>,
) -> Result<(), String> {
    if *compression > 9 || *compression < 1 {
        return Err(format!(
            "The compression value must be between 1 and 9, not {}",
            compression
        ));
    }

    let app = App::new()?;
    let (inst, inst_path) = app.parse_instruction(name)?;
    let tmp = path::tmp_dir()?;

    let tmp_data = &tmp.join("data");
    let tmp_inst = &tmp.join("instruction.yml");
    path::cp(inst_path, &tmp_inst, None)?;
    for (name, entries) in inst.save {
        let entry_dir = tmp_data.join(name);
        for add in &entries.add {
            path::cp(add, &entry_dir, Some(&entries.del))?;
        }
    }

    let path = match path {
        Some(path) => {
            let mut path = PathBuf::from(path);
            if path.is_dir() {
                path.push(format!("{name}.conman"));
            }
            path
        }
        None => app.configs(name)?.0.join(PathBuf::from(
            Local::now().format("%F %H:%M:%S.conman").to_string(),
        )),
    };
    archive::zip(&path, &[tmp_data, tmp_inst], compression, password.as_ref())?;
    path::rm(&tmp)?;

    Ok(())
}

fn run_load(path: &String, name: &Option<String>) -> Result<(), String> {
    todo!()
}

fn run_use(name: &String, number: &usize) -> Result<(), String> {
    todo!()
}

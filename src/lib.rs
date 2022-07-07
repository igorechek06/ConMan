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
use clap::Parser;
use std::path::PathBuf;
use util::{path, print_err, str_err};

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

    print_err(result).is_err() as i32
}

fn run_list(names: &Vec<String>) -> Result<(), String> {
    let app = App::new()?;
    let mut result = String::new();

    if app.instructions.is_empty() {
        return Err(format!("Instructions not found"));
    }

    for name in names {
        if !app.contains(name) {
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
    let app = App::new()?;
    let (inst_path, conf_path) = path::config_dir()?;

    for name in names {
        if app.contains(name) {
            print_err::<(), _>(Err(format!("Instruction already exist ({})", name)));
            continue;
        }

        let mut inst = inst_path.clone();
        let mut storage = conf_path.clone();
        inst.push(format!("{}.yml", name));
        storage.push(name);

        print_err(path::mkfile(inst));
        print_err(path::mkdir(storage));
    }

    Ok(())
}

fn run_del(names: &Vec<String>, number: &Option<usize>) -> Result<(), String> {
    let app = App::new()?;

    for name in names {
        if let Some(number) = number {
            let confs: Vec<&PathBuf> = app.config(name)?.1.values().collect();
            path::rm(
                confs
                    .get(number - 1)
                    .ok_or(format!("Config does not exist ({})", number))?,
            )?;
        } else {
            let inst = app.instruction(name)?;
            let confs = app.config(name)?;

            path::rm(inst)?;
            path::rm(&confs.0)?;
        }
    }

    Ok(())
}

fn run_edit(name: &String) -> Result<(), String> {
    str_err(open::that(App::new()?.instruction(&name)?))
}

fn run_save(
    name: &String,
    path: &Option<String>,
    compression: &u8,
    password: &Option<String>,
) -> Result<(), String> {
    let app = App::new()?;
    let (inst, inst_path) = app.parse_instruction(name)?;

    let tmp = path::tmp_dir()?;
    let data = path::add(&tmp, "data");

    path::cp(inst_path, path::add(&tmp, "instruction.yml"))?;

    for (name, entries) in inst.save {
        let entry_dir = path::add(&data, name);
        for entry in &entries.add {
            path::cp(entry, &entry_dir)?;
        }
    }

    path::rm(tmp)?;

    Ok(())
}

fn run_load(path: &String, name: &Option<String>) -> Result<(), String> {
    todo!()
}

fn run_use(name: &String, number: &usize) -> Result<(), String> {
    todo!()
}

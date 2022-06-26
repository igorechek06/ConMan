extern crate clap;
extern crate dirs;
extern crate open;
extern crate regex;
extern crate serde;
extern crate serde_yaml;

mod app;
mod args;
mod settings;
mod util;

use app::App;
use args::{Action, Args};
use clap::Parser;
use util::{err, path};

/*
TODO: Add color output
TODO: Add i18n
*/

pub fn run() -> i32 {
    let args = Args::parse();
    let result = match &args.action {
        Action::Help => Ok(()),
        Action::List { name } => run_list(name),
        Action::Add { name } => run_add(name),
        Action::Del { name } => run_del(name),
        Action::Edit { name } => run_edit(name),
        Action::Save { name, path } => run_save(name, path),
        Action::Load { file, name } => run_load(file, name),
        Action::Use { name, number } => run_use(name, number),
    };

    err(result)
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
    let inst_path = path::get("CONMAN_INSTRUCTIONS")?;
    let conf_path = path::get("CONMAN_CONFIGS")?;

    for name in names {
        if app.contains(name) {
            err(Err(format!("Instruction already exist ({})", name)));
            continue;
        }

        let mut inst = inst_path.clone();
        let mut storage = conf_path.clone();
        inst.push(format!("{}.yml", name));
        storage.push(name);

        err(path::mkfile(inst));
        err(path::mkdir(storage));
    }

    Ok(())
}

fn run_del(names: &Vec<String>) -> Result<(), String> {
    let app = App::new()?;

    for name in names {
        if !app.contains(name) {
            err(Err(format!("Instruction does not exist ({})", name)));
            continue;
        }

        let inst = &app.instructions[name];
        let confs = &app.configs[name];

        err(path::rm(inst));
        err(path::rm(&confs.0));
    }

    Ok(())
}

fn run_edit(name: &String) -> Result<(), String> {
    open::that(
        App::new()?
            .instructions
            .get(name)
            .ok_or(format!("Instruction does not exist ({})", name))?,
    )
    .or(Err(format!("Can't open file in system editor ({})", name)))
}

fn run_save(name: &String, path: &Option<String>) -> Result<(), String> {
    println!("{:?} {:?}", name, path);
    Ok(())
}

fn run_load(path: &String, name: &Option<String>) -> Result<(), String> {
    println!("{:?} {:?}", path, name);
    Ok(())
}

fn run_use(name: &String, number: &usize) -> Result<(), String> {
    println!("{} {}", name, number);
    Ok(())
}

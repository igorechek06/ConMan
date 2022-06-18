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
        Action::List { name } => run_list(name, &args),
        Action::Add { name } => run_add(name, &args),
        Action::Del { name } => run_del(name, &args),
        Action::Edit { name } => run_edit(name, &args),
        Action::Save { name, path } => run_save(name, path.as_ref(), &args),
        Action::Load { file, name } => run_load(file, name.as_ref(), &args),
    };

    err(result)
}

fn run_list(names: &Vec<String>, args: &Args) -> Result<(), String> {
    let app = App::new(&args)?;
    let instructions = app.list(names)?;
    let mut result = String::new();

    if instructions.is_empty() {
        return Err("Instructions not found".to_string());
    }

    for (inst_name, configs) in instructions {
        result += format!("\n{}\n", inst_name).as_str();

        for conf_name in configs {
            result += format!("  {}\n", conf_name).as_str();
        }
    }

    println!("{}", result.trim());

    Ok(())
}

fn run_add(names: &Vec<String>, args: &Args) -> Result<(), String> {
    let app = App::new(args)?;
    let inst_path = path::get("CONMAN_INSTRUCTIONS")?;
    let conf_path = path::get("CONMAN_CONFIGS")?;

    for name in names {
        if app.contains(name)? {
            err(Err(format!("Instructions already exists ({})", name)));
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

fn run_del(names: &Vec<String>, args: &Args) -> Result<(), String> {
    let app = App::new(args)?;

    for name in names {
        if !app.contains(name)? {
            err(Err(format!("Instructions does not exists ({})", name)));
            continue;
        }

        let (inst, storage) = app.get(name)?;

        err(path::rm(inst));
        err(path::rm(storage.0));
    }

    Ok(())
}

fn run_edit(name: &String, args: &Args) -> Result<(), String> {
    open::that(App::new(args)?.get(name)?.0)
        .or(Err(format!("Can't open file in system editor ({})", name)))
}

fn run_save(name: &String, path: Option<&String>, args: &Args) -> Result<(), String> {
    println!("{} {:?} {}", name, path, args.force);
    Ok(())
}

fn run_load(path: &String, name: Option<&String>, args: &Args) -> Result<(), String> {
    println!("{:?} {:?} {}", path, name, args.force);
    Ok(())
}

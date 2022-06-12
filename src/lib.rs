extern crate clap;
extern crate dirs;
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

use crate::util::get_file_name;

/*
TODO: Add color output
TODO: Add locales
*/

pub fn run() -> i32 {
    let args = Args::parse();
    let result = match &args.action {
        Action::Help => Ok(()),
        Action::List => run_list(&args),
        Action::Add { name } => run_add(name, &args),
        Action::Del { name } => run_del(name, &args),
        Action::Edit { name } => run_edit(name, &args),
        Action::Save { name, path } => run_save(name, path, &args),
        Action::Load { file, name } => run_load(file, name, &args),
    };

    return match result {
        Ok(..) => 0,
        Err(error) => {
            eprintln!("Error :: {}", error);
            1
        }
    };
}

fn run_list(args: &Args) -> Result<(), String> {
    let app = App::new()?;
    let mut output = String::new();

    if !app.instructions.is_empty() {
        output += "Instructions\n\n";
        for (path, inst_name) in app.instructions {
            let inst_name = inst_name
                .name
                .unwrap_or(get_file_name(path)?.0);
            output += format!("{}:\n", inst_name).as_str();
            for (conf_name, configs) in &app.configs {
                if *conf_name == inst_name {
                    for conf in configs {
                        output += format!("  - {}\n", get_file_name(conf)?.0).as_str();
                    }
                }
            }
        }
    } else {
        return Err("Instructions not found".to_string());
    }

    print!("{}", output);
    Ok(())
}

fn run_add(name: &String, args: &Args) -> Result<(), String> {
    println!("{} {}", name, args.force);
    Ok(())
}

fn run_del(name: &String, args: &Args) -> Result<(), String> {
    println!("{} {}", name, args.force);
    Ok(())
}

fn run_edit(name: &String, args: &Args) -> Result<(), String> {
    println!("{} {}", name, args.force);
    Ok(())
}

fn run_save(name: &String, path: &Option<String>, args: &Args) -> Result<(), String> {
    println!("{} {:?} {}", name, path, args.force);
    Ok(())
}

fn run_load(path: &String, name: &Option<String>, args: &Args) -> Result<(), String> {
    println!("{:?} {:?} {}", path, name, args.force);
    Ok(())
}

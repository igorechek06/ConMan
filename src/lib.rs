extern crate clap;
extern crate serde;

mod app;
mod args;
mod settings;
mod util;

use app::App;
use args::{Action, Args};
use clap::Parser;

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

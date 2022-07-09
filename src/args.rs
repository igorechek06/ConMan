use clap::{Parser, Subcommand};

pub mod src {
    pub const LONG_ABOUT: &str = "";

    pub mod help {
        pub mod actions {
            pub const LIST: &str = "Show all instructions";
            pub const ADD: &str = "Add instruction";
            pub const DEL: &str = "Delete instruction";
            pub const EDIT: &str = "Edit instruction";
            pub const SAVE: &str = "Save config to archive";
            pub const LOAD: &str = "Load config from archive";
            pub const USE: &str = "Use config in the system";
        }

        pub mod fields {
            pub const NAME: &str = "Instruction name";
            pub const PATH: &str = "Path where you want to save the archive";
            pub const FILE: &str = "Path to archive";
            pub const NUMBER: &str = "Number of config";
            pub const COMPRESSION: &str = "Archive compression";
            pub const PASSWORD: &str = "Archive password";
        }

        pub mod flags {
            pub const FORCE: &str = "Never ask, always do";
            pub const VERBOSE: &str = "Show debug information";
        }

        pub const HELP: &str = "Show help information";
    }
}

#[derive(Parser)]
#[clap(about, long_about=src::LONG_ABOUT)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
    #[clap(long, short, help=src::help::flags::FORCE)]
    pub force: bool,
    #[clap(long, short, help=src::help::flags::VERBOSE)]
    pub verbose: bool,
    #[clap(long, short, help=src::help::HELP)]
    pub help: bool,
}

#[derive(Subcommand)]
pub enum Action {
    #[clap(about=src::help::HELP)]
    Help,

    // Instructions manager
    #[clap(about=src::help::actions::LIST)]
    List {
        #[clap(help=src::help::fields::NAME)]
        names: Vec<String>,
    },
    #[clap(about=src::help::actions::ADD)]
    Add {
        #[clap(required=true, help=src::help::fields::NAME)]
        names: Vec<String>,
    },
    #[clap(about=src::help::actions::DEL)]
    Del {
        #[clap(required=true, help=src::help::fields::NAME)]
        names: Vec<String>,
        #[clap(short, long, help=src::help::fields::NUMBER)]
        number: Vec<usize>,
    },
    #[clap(about=src::help::actions::EDIT)]
    Edit {
        #[clap(help=src::help::fields::NAME)]
        name: String,
    },

    // Config manager
    #[clap(about=src::help::actions::SAVE)]
    Save {
        #[clap(help=src::help::fields::NAME)]
        name: String,
        #[clap(help=src::help::fields::PATH)]
        path: Option<String>,
        #[clap(short, long, default_value_t=9, help=src::help::fields::COMPRESSION)]
        compression: u8,
        #[clap(short, long, help=src::help::fields::PASSWORD)]
        password: Option<String>,
    },
    #[clap(about=src::help::actions::LOAD)]
    Load {
        #[clap(help=src::help::fields::FILE)]
        file: String,
        #[clap(help=src::help::fields::NAME)]
        name: Option<String>,
        #[clap(short, long, help=src::help::fields::PASSWORD)]
        password: Option<String>,
    },
    #[clap(about=src::help::actions::USE)]
    Use {
        #[clap(help=src::help::fields::NAME)]
        name: String,
        #[clap(help=src::help::fields::NUMBER, default_value_t=1)]
        number: usize,
        #[clap(short, long, help=src::help::fields::PASSWORD)]
        password: Option<String>,
    },
}

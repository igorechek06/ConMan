use clap::{Parser, Subcommand};

pub mod src {
    pub const LONG_ABOUT: &str = "";

    pub mod help {
        pub const LIST: &str = "Show all instructions";
        pub const ADD: &str = "Add instruction";
        pub const DEL: &str = "Delete instruction";
        pub const EDIT: &str = "Edit instruction";

        pub const SAVE: &str = "Save config to archive";
        pub const LOAD: &str = "Load config from archive";

        pub const NAME: &str = "Instruction name";
        pub const PATH: &str = "Path where you want to save the archive";
        pub const FILE: &str = "Path to archive";

        pub const FORCE: &str = "Never ask, always do";
        pub const VERBOSE: &str = "Show debug information";
        pub const HELP: &str = "Show help information";
    }
}

#[derive(Parser)]
#[clap(about, long_about=src::LONG_ABOUT)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
    #[clap(long, short, help=src::help::FORCE)]
    pub force: bool,
    #[clap(long, short, help=src::help::VERBOSE)]
    pub verbose: bool,
    #[clap(long, short, help=src::help::HELP)]
    pub help: bool,
}

#[derive(Subcommand)]
pub enum Action {
    #[clap(about=src::help::HELP)]
    Help,

    // Instructions manager
    #[clap(about=src::help::LIST)]
    List,
    #[clap(about=src::help::ADD)]
    Add {
        #[clap(help=src::help::NAME)]
        name: String,
    },
    #[clap(about=src::help::DEL)]
    Del {
        #[clap(help=src::help::NAME)]
        name: String,
    },
    #[clap(about=src::help::EDIT)]
    Edit {
        #[clap(help=src::help::NAME)]
        name: String,
    },

    // Config manager
    #[clap(about=src::help::SAVE)]
    Save {
        #[clap(help=src::help::NAME)]
        name: String,
        #[clap(help=src::help::PATH)]
        path: Option<String>,
    },
    #[clap(about=src::help::LOAD)]
    Load {
        #[clap(help=src::help::FILE)]
        file: String,
        #[clap(help=src::help::NAME)]
        name: Option<String>,
    },
}

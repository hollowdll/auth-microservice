use clap::{
    Parser,
    Subcommand,
};

/// CLI client to interact with the authentication microservice.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Login with username and password.
    Login {
        #[arg(long)]
        rest: bool,
    },
}
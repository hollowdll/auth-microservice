use clap::{
    Parser,
    Subcommand,
    Args,
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
    Login(LoginArgs),
}

#[derive(Args)]
pub struct LoginArgs {
    /// Use REST API instead of gRPC.
    #[arg(short, long)]
    pub rest: bool,
}
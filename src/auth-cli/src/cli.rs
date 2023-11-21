use clap::{
    Parser,
    Subcommand,
    Args,
};

/// CLI client to interact with the authentication microservice
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Login with username and password
    Login(LoginArgs),
    /// User commands
    User {
        #[command(subcommand)]
        command: Option<UserCommands>,
    }
}

#[derive(Args)]
pub struct UserArgs {
    #[command(subcommand)]
    pub command: Option<UserCommands>,
}

#[derive(Args)]
pub struct LoginArgs {}

#[derive(Subcommand)]
pub enum UserCommands {
    /// List users. Uses gRPC by default.
    Ls(ListUserArgs)
}

#[derive(Args)]
pub struct ListUserArgs {
    /// Use REST API instead of gRPC
    #[arg(long)]
    pub rest: bool,

    /// Hide results.
    #[arg(long)]
    pub hide: bool,
}
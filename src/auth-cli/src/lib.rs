use cli::Cli;
use grpc_client::GrpcClient;
use cli::Commands;

pub mod grpc_client;
pub mod cli;
pub mod user;

pub async fn run(cli: &Cli, grpc_client: &GrpcClient) {
    match cli.command {
        Some(Commands::Login { rest }) => {
            if !rest {
                println!("Using gRPC");
            }
            println!("Login with username and password");
        }
        None => println!("No command given. Use --help for list of commands")
    }
}
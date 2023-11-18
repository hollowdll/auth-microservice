use cli::Cli;
use grpc_client::GrpcClient;
use cli::Commands;
use std::io::{self, Write};
use crate::grpc_client::auth::LoginRequest;

pub mod grpc_client;
pub mod cli;
pub mod user;

pub async fn run(cli: &Cli, grpc_client: &mut GrpcClient) {
    match cli.command {
        Some(Commands::Login { rest }) => {
            if !rest {
                println!("Using gRPC");
            }
            println!("Login with username and password");

            let username = match ask_user_input("Username: ") {
                Ok(username) => username,
                Err(_) => return,
            };
            let password = match ask_user_input("Password: ") {
                Ok(password) => password,
                Err(_) => return,
            };

            let login_request = LoginRequest {
                username,
                password,
            };

            match grpc_client.login(login_request).await {
                Ok(response) => {
                    println!("success");
                },
                Err(e) => eprintln!("Failed to login: {}", e)
            }
        }
        None => println!("No command given. Use --help for list of commands")
    }
}

fn ask_user_input(text_to_ask: &str) -> io::Result<String> {
    let mut input = String::new();

    print!("{text_to_ask}");
    io::stdout().flush().expect("Unexpected I/O error");
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Failed to read line: {}", e);
        return Err(e);
    }
    let input = input.trim().to_string();

    Ok(input)
}
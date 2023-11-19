use cli::{Cli, UserCommands};
use grpc_client::GrpcClient;
use cli::Commands;
use std::io::{self, Write};
use crate::grpc_client::auth::LoginRequest;

pub mod grpc_client;
pub mod cli;
pub mod user;
pub mod storage;

pub async fn run(cli: &Cli, grpc_client: &mut GrpcClient) {
    match &cli.command {
        Some(Commands::Login(_args)) => {
            println!("Login with username and password");

            let username = match ask_user_input("Username: ") {
                Ok(username) => username,
                Err(e) => return eprintln!("Failed to read username: {}", e),
            };
            let password = match rpassword::prompt_password("Password: ") {
                Ok(password) => password,
                Err(e) => return eprintln!("Failed to read password: {}", e),
            };

            let login_request = LoginRequest {
                username,
                password,
            };

            match grpc_client.login(login_request).await {
                Ok(response) => {
                    println!("{}", response.get_ref().message.as_str());
                },
                Err(e) => eprintln!("Failed to login: {}", e)
            }
        }
        Some(Commands::User { command }) => {
            match command {
                Some(UserCommands::Ls(_args)) => {
                    match grpc_client.get_users().await {
                        Ok(response) => {
                            let users = &response.get_ref().users;

                            for user in users {
                                println!("{{");
                                println!("  ID: {}", user.id);
                                println!("  Username: {}", user.username);
                                println!("  Roles: {:?}", user.roles);
                                println!("}}");
                            }
                        },
                        Err(e) => eprintln!("Failed to list users: {}", e)
                    }
                }
                None => return
            }
        }
        None => println!("No command given. Use --help for list of commands")
    }
}

fn ask_user_input(text_to_ask: &str) -> io::Result<String> {
    let mut input = String::new();

    print!("{}", text_to_ask);
    io::stdout().flush()?;
    if let Err(e) = io::stdin().read_line(&mut input) {
        return Err(e);
    }
    let input = input.trim().to_string();

    Ok(input)
}
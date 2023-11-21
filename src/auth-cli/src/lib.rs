use cli::{Cli, UserCommands};
use grpc::GrpcClient;
use cli::Commands;
use http::HttpClient;
use crate::grpc::auth::LoginRequest;
use util::ask_user_input;

pub mod grpc;
pub mod http;
pub mod cli;
pub mod user;
pub mod storage;
pub mod util;

/// Runs the program and handles the passed commands.
pub async fn run(cli: &Cli, grpc_client: &mut GrpcClient, http_client: &HttpClient) {
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

            let response = match grpc_client.login(login_request).await {
                Ok(response) => response,
                Err(e) => return eprintln!("{}", e)
            };
            println!("{}", response.get_ref().message.as_str());
        }
        Some(Commands::User { command }) => {
            match command {
                Some(UserCommands::Ls(args)) => {
                    let mut count: u64 = 1;

                    // use REST API instead of gRPC
                    if args.rest {
                        let data = match http_client.get_users().await {
                            Ok(data) => data,
                            Err(e) => return eprintln!("{}", e),
                        };

                        return for user in data {
                            println!("{}: {}", count, user);
                            count += 1;
                        }
                    }

                    let response = match grpc_client.get_users().await {
                        Ok(response) => response,
                        Err(e) => return eprintln!("{}", e)
                    };
                    let users = &response.get_ref().users;

                    for user in users {
                        println!("{}: {}", count, user);
                        count += 1;
                    }
                }
                None => return
            }
        }
        None => println!("No command given. Use --help for list of commands")
    }
}
use cli::{Cli, UserCommands};
use grpc::GrpcClient;
use cli::Commands;
use http::HttpClient;
use util::{
    ask_user_input,
    print_using_grpc,
    print_using_rest,
};

pub mod grpc;
pub mod http;
pub mod cli;
pub mod user;
pub mod storage;
pub mod util;

/// Runs the program and handles the passed commands.
pub async fn run(cli: &Cli, grpc_client: &mut GrpcClient, http_client: &HttpClient) {
    match &cli.command {
        Some(Commands::Login(args)) => {
            println!("Login with username and password");

            let username = match ask_user_input("Username: ") {
                Ok(username) => username,
                Err(e) => return eprintln!("Failed to read username: {}", e),
            };
            let password = match rpassword::prompt_password("Password: ") {
                Ok(password) => password,
                Err(e) => return eprintln!("Failed to read password: {}", e),
            };

            if args.rest {
                print_using_rest();
                if let Err(e) = http_client.login(username, password).await {
                    return eprintln!("{}", e);
                };
            } else {
                print_using_grpc();
                if let Err(e) = grpc_client.login(username, password).await {
                    return eprintln!("{}", e)
                };
            }

            println!("Login succeeded");
        }
        Some(Commands::User { command }) => {
            match command {
                Some(UserCommands::Ls(args)) => {
                    let mut count: u64 = 1;

                    // use REST API instead of gRPC
                    if args.rest {
                        print_using_rest();
                        let data = match http_client.get_users().await {
                            Ok(data) => data,
                            Err(e) => return eprintln!("{}", e),
                        };
                        
                        println!("User count: {}", data.len());
                        if args.hide {
                            return
                        }

                        return for user in &data {
                            println!("{}: {}", count, user);
                            count += 1;
                        }
                    }

                    print_using_grpc();
                    let response = match grpc_client.get_users().await {
                        Ok(response) => response,
                        Err(e) => return eprintln!("{}", e)
                    };
                    let data = &response.get_ref().users;

                    println!("User count: {}", data.len());
                    if args.hide {
                        return
                    }

                    for user in data {
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
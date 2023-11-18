use std::error::Error;
use clap::Parser;
use auth_cli::{
    cli::Cli,
    grpc_client::GrpcClient,
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut grpc_client = GrpcClient::build().await?;

    auth_cli::run(&cli, &mut grpc_client).await;

    Ok(())
}

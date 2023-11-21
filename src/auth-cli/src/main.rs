use std::error::Error;
use clap::Parser;
use auth_cli::{
    cli::Cli,
    grpc::GrpcClient,
    http::HttpClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut grpc_client = GrpcClient::build().await?;
    let http_client = HttpClient::build();

    auth_cli::run(&cli, &mut grpc_client, &http_client).await;

    Ok(())
}

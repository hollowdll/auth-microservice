use std::env;
use auth::{
    LoginRequest,
    login_client::LoginClient,
};
use std::error::Error;
use tonic::transport::Channel;

pub mod auth {
    tonic::include_proto!("auth_service");
}

pub struct GrpcClient {
    pub client: LoginClient<Channel>
}

impl GrpcClient {
    pub async fn build() -> Result<GrpcClient, Box<dyn Error>> {
        let api_url = env::var("GRPC_API_URL")
            .expect("GRPC_API_URL environment variable is not set");

        Ok(GrpcClient {
            client: LoginClient::connect(api_url).await?
        })
    }
}
use std::env;
use auth::{
    LoginRequest,
    login_client::LoginClient,
};
use std::error::Error;
use tonic::{transport::Channel, Response};
use self::auth::LoginResponse;
use crate::storage::store_jwt;

pub mod auth {
    tonic::include_proto!("auth_service");
}

/// gRPC client to make gRPC calls.
pub struct GrpcClient {
    pub client: LoginClient<Channel>
}

impl GrpcClient {
    /// Builds a new instance of `GrpcClient`.
    pub async fn build() -> Result<GrpcClient, Box<dyn Error>> {
        let api_url = env::var("GRPC_API_URL")
            .expect("GRPC_API_URL environment variable is not set");

        Ok(GrpcClient {
            client: LoginClient::connect(api_url).await?
        })
    }

    /// Login with username and password.
    /// 
    /// TODO: Returns
    pub async fn login(&mut self, login_request: LoginRequest) -> Result<Response<LoginResponse>, Box<dyn Error>> {
        let request = tonic::Request::new(login_request);
        let response = match self.client.login_user(request).await {
            Ok(response) => response,
            Err(e) => {
                return Err(e.message().into());
            }
        };

        let jwt = response.metadata()
            .get("Authorization")
            .unwrap()
            .as_bytes();

        if let Err(e) = store_jwt(jwt) {
            return Err(format!("Error storing JWT: {}", e).into());
        }

        Ok(response)
    }
}
use std::env;
use auth::{
    LoginRequest,
    GetUsersRequest,
    login_client::LoginClient,
    user_client::UserClient,
};
use std::error::Error;
use tonic::{transport::Channel, Response};
use self::auth::{LoginResponse, GetUsersResponse};
use crate::storage::store_jwt;

pub mod auth {
    tonic::include_proto!("auth_service");
}

/// gRPC client to make gRPC calls.
pub struct GrpcClient {
    pub login_client: LoginClient<Channel>,
    pub user_client: UserClient<Channel>,
}

impl GrpcClient {
    /// Builds a new instance of `GrpcClient`.
    pub async fn build() -> Result<GrpcClient, Box<dyn Error>> {
        let api_url = env::var("GRPC_API_URL")
            .expect("GRPC_API_URL environment variable is not set");

        Ok(GrpcClient {
            login_client: LoginClient::connect(api_url.clone()).await?,
            user_client: UserClient::connect(api_url).await?,
        })
    }

    /// gRPC call to login with username and password.
    /// Stores received JWT access token to file if success.
    /// 
    /// Returns the response.
    pub async fn login(&mut self, login_request: LoginRequest) -> Result<Response<LoginResponse>, Box<dyn Error>> {
        let request = tonic::Request::new(login_request);
        let response = match self.login_client.login_user(request).await {
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

    /// gRPC call to get users from the user database.
    /// 
    /// Returns the users.
    pub async fn get_users(&mut self) -> Result<Response<GetUsersResponse>, Box<dyn Error>> {
        let request = tonic::Request::new(GetUsersRequest {});
        let response = match self.user_client.get_users(request).await {
            Ok(response) => response,
            Err(e) => {
                return Err(e.message().into());
            }
        };

        Ok(response)
    }
}
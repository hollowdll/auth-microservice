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
use crate::{
    storage::{
        store_jwt,
        get_jwt,
    },
    util::print_response_time,
};
use std::time::Instant;

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
    pub async fn login(&mut self, username: String, password: String) -> Result<Response<LoginResponse>, Box<dyn Error>> {
        let request = tonic::Request::new(LoginRequest {
            username,
            password,
        });
        let now = Instant::now();
        let response = match self.login_client.login_user(request).await {
            Ok(response) => {
                print_response_time(&now);
                response
            },
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
        let jwt = match get_jwt() {
            Ok(jwt) => jwt,
            Err(e) => return Err(format!("Error getting JWT: {}", e).into()),
        };

        let mut request = tonic::Request::new(GetUsersRequest {});
        request.metadata_mut().insert("authorization", jwt.parse().unwrap());

        let now = Instant::now();
        let response = match self.user_client.get_users(request).await {
            Ok(response) => {
                print_response_time(&now);
                response
            },
            Err(e) => {
                match e.code() {
                    tonic::Code::PermissionDenied => {
                        return Err(format!("Permission denied: {}", e.code()).into());
                    }
                    tonic::Code::Unauthenticated => {
                        return Err(format!("Unauthenticated: {}", e.code()).into());
                    }
                    _ => return Err(format!("{}", e.code()).into()),
                }
            }
        };

        Ok(response)
    }
}
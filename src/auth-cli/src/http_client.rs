use reqwest::Client;
use std::error::Error;
use std::env;

use crate::user::UserData;

/// HTTP client to make network requests to the REST API.
pub struct HttpClient {
    pub client: Client,
    pub api_url: String,
}

impl HttpClient {
    pub fn build() -> HttpClient {
        let api_url = env::var("GRPC_API_URL")
            .expect("REST_API_URL environment variable is not set");

        HttpClient {
            client: Client::new(),
            api_url,
        }
    }

    pub async fn get_users(&self) -> Result<Vec<UserData>, Box<dyn Error>> {
        let response = self.client
            .get("/api/v1/users")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to get users: {}", response.status()).into());
        }
        let data = response.json::<Vec<UserData>>().await?;

        Ok(data)
    }
}
use reqwest::Client;
use std::{
    time::Instant,
    error::Error,
    env,
};
use crate::{
    storage::get_jwt,
    util::print_response_time,
};
use crate::user::UserData;

/// HTTP client to make network requests to the REST API.
pub struct HttpClient {
    pub client: Client,
    pub api_url: String,
}

impl HttpClient {
    pub fn build() -> HttpClient {
        let api_url = env::var("REST_API_URL")
            .expect("REST_API_URL environment variable is not set");

        HttpClient {
            client: Client::new(),
            api_url,
        }
    }

    pub async fn get_users(&self) -> Result<Vec<UserData>, Box<dyn Error>> {
        let jwt = match get_jwt() {
            Ok(jwt) => jwt,
            Err(e) => return Err(format!("Error getting JWT: {}", e).into()),
        };
        let now = Instant::now();
        let response = self.client
            .get(format!("{}/api/v1/users", self.api_url))
            .header("Authorization", jwt)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to get users: {}", response.status()).into());
        }
        print_response_time(&now);
        let data = response.json::<Vec<UserData>>().await?;

        Ok(data)
    }
}
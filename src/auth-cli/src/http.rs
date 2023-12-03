use reqwest::Client;
use std::{
    time::Instant,
    error::Error,
    env,
};
use crate::{
    storage::{
        get_jwt,
        store_jwt,
    },
    util::print_response_time,
};
use crate::user::UserData;
use serde::Serialize;

/// HTTP client to make network requests to the REST API.
pub struct HttpClient {
    pub client: Client,
    pub api_url: String,
}

#[derive(Serialize)]
pub struct LoginRequest {
    username: String,
    password: String,
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

    /// Login with username and password.
    /// Stores received JWT access token to file if success.
    pub async fn login(&self, username: String, password: String) -> Result<(), Box<dyn Error>> {
        let request = LoginRequest {
            username,
            password
        };
        let now = Instant::now();
        let response = self.client
            .post(format!("{}/auth/login", self.api_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("{} {:?}", response.status(), response.text().await?).into());
        }
        print_response_time(&now);

        let jwt = match response.headers().get("Authorization") {
            Some(jwt) => jwt,
            None => return Err("No JWT received".into()),
        };

        if let Err(e) = store_jwt(jwt.as_bytes()) {
            return Err(format!("Error storing JWT: {}", e).into());
        }

        Ok(())
    }

    /// Get users from the user database.
    pub async fn get_users(&self) -> Result<Vec<UserData>, Box<dyn Error>> {
        let jwt = match get_jwt() {
            Ok(jwt) => jwt,
            Err(e) => return Err(format!("Error getting JWT: {}", e).into()),
        };
        let now = Instant::now();
        let response = self.client
            .get(format!("{}/users", self.api_url))
            .header("Authorization", jwt)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("{}", response.status()).into());
        }
        let data = response.json::<Vec<UserData>>().await?;
        print_response_time(&now);

        Ok(data)
    }
}
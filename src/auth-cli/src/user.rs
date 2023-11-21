use std::fmt::Display;
use serde::Deserialize;
use crate::grpc::auth;

#[derive(Deserialize)]
pub struct UserData {
    pub id: String,
    pub username: String,
    pub roles: Vec<String>,
}

impl Display for UserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n  ID: {}\n  Username: {}\n  Roles: {:?}\n}}",
            self.id,
            self.username,
            self.roles,
        )
    }
}

impl Display for auth::UserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n  ID: {}\n  Username: {}\n  Roles: {:?}\n}}",
            self.id,
            self.username,
            self.roles,
        )
    }
}
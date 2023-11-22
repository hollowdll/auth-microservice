use std::fmt::Display;
use serde::Deserialize;
use crate::grpc::auth;
use prost_types::Timestamp;
use chrono::{DateTime, Utc, TimeZone};

#[derive(Deserialize)]
pub struct UserData {
    pub id: String,
    pub username: String,
    pub roles: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

impl Display for UserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n  ID: {}\n  Username: {}\n  Roles: {:?}\n  Created: {}\n}}",
            self.id,
            self.username,
            self.roles,
            string_to_iso8601(self.created_at.as_ref()),
        )
    }
}

impl Display for auth::UserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n  ID: {}\n  Username: {}\n  Roles: {:?}\n  Created: {}\n}}",
            self.id,
            self.username,
            self.roles,
            prost_timestamp_to_iso8601(self.created_at.as_ref().unwrap()),
        )
    }
}

/// Convert prost timestamp to ISO8601 timestamp string.
fn prost_timestamp_to_iso8601(timestamp: &Timestamp) -> String {
    Utc.timestamp_opt(timestamp.seconds, timestamp.nanos as u32)
        .unwrap()
        .to_rfc3339()
}

/// Convert string date time to ISO8601 timestamp string.
fn string_to_iso8601(s: &str) -> String {
    DateTime::parse_from_rfc3339(s)
        .unwrap()
        .with_timezone(&Utc)
        .to_rfc3339()
}
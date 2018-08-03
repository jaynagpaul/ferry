use chrono::{DateTime, Utc};
use serde_json::Value;

/// User contains the information common amongst most OAuth and OAuth2 providers.
/// All of the "raw" datafrom the provider can be found in the `RawData` field.
#[derive(Serialize, Deserialize)]
pub struct User {
    pub raw_data: Value,
    pub provider: &'static str,
    pub email: &'static str,
    pub name: &'static str,
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub nickname: &'static str,
    pub description: &'static str,
    pub user_id: &'static str,
    pub avatar_url: &'static str,
    pub location: &'static str,
    pub access_token: &'static str,
    pub access_token_secret: &'static str,
    pub refresh_token: &'static str,
    pub expires_at: DateTime<Utc>,
}

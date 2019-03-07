//! # Sleep Goals APIs

use crate::UserId;

use chrono::DateTime;
use serde_derive::Deserialize;
use url::Url;

pub mod get;
pub mod update;

/// Generate a goals endpoint URL for the user's daily sleep goals
pub fn url(user: UserId) -> Url {
    Url::parse(&format!("https://api.fitbit.com/1.2/user/{}/sleep/goal.json", user.to_string())).unwrap()
}

/// Information about the current sleep goal.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub min_duration: usize,
    // TODO: Not sure what the correct timezone is here...
    pub updated_on: DateTime<chrono::Utc>,
}

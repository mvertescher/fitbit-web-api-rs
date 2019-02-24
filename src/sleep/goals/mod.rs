//! # Sleep Goals APIs

use chrono::DateTime;
use serde_derive::Deserialize;

pub mod get;
pub mod update;

pub const URL: &str = "https://api.fitbit.com/1.2/user/-/sleep/goal.json";

/// Information about the current sleep goal.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub min_duration: usize,
    // TODO: Not sure what the correct timezone is here...
    pub updated_on: DateTime<chrono::Utc>,
}

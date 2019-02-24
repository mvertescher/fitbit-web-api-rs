//! Update sleep goals.

use serde_derive::{Deserialize, Serialize};

/// Sleep goal update request.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// Target sleep duration in minutes.
    pub min_duration: usize,
}

/// Sleep goal update response.
#[derive(Deserialize, Debug)]
pub struct Response {
    pub goal: super::Goal,
}

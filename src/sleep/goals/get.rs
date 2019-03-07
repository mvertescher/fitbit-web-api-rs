//! Get a user's current sleep goals.

use std::fmt;

use serde_derive::Deserialize;

/// Get sleep goals response.
#[derive(Deserialize, Debug)]
pub struct Response {
    pub consistency: Consistency,
    pub goal: super::Goal,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

// TODO: not sure what this is?
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Consistency {
    pub flow_id: usize,
}

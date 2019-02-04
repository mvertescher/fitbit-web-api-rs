//! Definitions for activity and exercise logs

use std::fmt;

use serde_derive::{Deserialize, Serialize};

pub mod goals;
pub mod lifetime_stats;
pub mod summary;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActivityLog {
    activity_id: usize,
    activity_parent_id: usize,
    calories: usize,
    description: String,
    distance: f32,
    duration: usize,
    has_start_time: bool,
    is_favorite: bool,
    log_id: usize,
    name: String,
    start_time: String,
    steps: usize,
}

impl fmt::Display for ActivityLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

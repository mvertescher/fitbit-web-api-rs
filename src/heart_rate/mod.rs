//! Heart rate data APIs

use serde_derive::Deserialize;

pub mod time_series;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeartRateZone {
    pub calories_out: f32,
    pub max: usize,
    pub min: usize,
    pub minutes: usize,
    pub name: String,
}

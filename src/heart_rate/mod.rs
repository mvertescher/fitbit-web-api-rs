//! Heart rate data APIs

use serde_derive::Deserialize;

pub mod intraday_time_series;
pub mod time_series;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeartRateZone {
    pub calories_out: Option<f32>,
    pub max: usize,
    pub min: usize,
    pub minutes: Option<usize>,
    pub name: String,
}

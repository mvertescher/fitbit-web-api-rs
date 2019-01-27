//! Update a profile for a user.

use chrono::naive::NaiveDate;
use serde_derive::Serialize;

/// Possible user profile options to change.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub gender: Option<super::Gender>,
    pub birthday: Option<NaiveDate>,
    pub height: Option<f32>,
    pub about_me: Option<String>,
    pub fullname: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub stride_length_walking: Option<f32>,
    pub stride_length_running: Option<f32>,
    pub weight_unit: Option<super::WeightUnit>,
    pub height_unit: Option<super::HeightUnit>,
    pub water_unit: Option<super::WaterUnit>,
    pub glucose_unit: Option<super::GlucoseUnit>,
    pub timezone: Option<String>,
    pub foods_locale: Option<String>,
    pub locale: Option<String>,
    pub locale_lang: Option<String>,
    pub locale_country: Option<String>,
    pub start_day_of_the_week: Option<super::StartDayOfTheWeek>,
    pub clock_time_display_format: Option<super::ClockTimeDisplayFormat>,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            gender: None,
            birthday: None,
            height: None,
            about_me: None,
            fullname: None,
            country: None,
            state: None,
            city: None,
            stride_length_walking: None,
            stride_length_running: None,
            weight_unit: None,
            height_unit: None,
            water_unit: None,
            glucose_unit: None,
            timezone: None,
            foods_locale: None,
            locale: None,
            locale_lang: None,
            locale_country: None,
            start_day_of_the_week: None,
            clock_time_display_format: None,
        }
    }
}

//! Get activity time series data.

use crate::{Period, UserId};

use chrono::naive::NaiveDate;
use serde_derive::Deserialize;
use strum_macros::*;
use url::Url;

#[derive(Debug, PartialEq, ToString)]
#[strum(serialize_all = "snake_case")]
pub enum Resource {
    Calories,
    #[strum(serialize = "caloriesBMR")]
    CaloriesBMR,
    Steps,
    Distance,
    Floors,
    Elevation,
    #[strum(serialize = "minutesSedentary")]
    Sedentary,
    #[strum(serialize = "minutesLightlyActive")]
    LightlyActive,
    #[strum(serialize = "minutesFairlyActive")]
    FairlyActive,
    #[strum(serialize = "minutesVeryActive")]
    VeryActive,
    #[strum(serialize = "activityCalories")]
    ActivityCalories,
}

/// Generate the request URL from a resource.
pub fn url_from_date_and_period(
    user_id: UserId,
    resource: &Resource,
    start: NaiveDate,
    period: Period,
) -> Url {
    Url::parse(&format!(
        "https://api.fitbit.com/1/user/{}/activities/{}/date/{}/{}.json",
        &user_id.to_string(),
        &resource.to_string(),
        start,
        &period.to_string()
    ))
    .unwrap()
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub date_time: NaiveDate,
    pub value: String,
}

macro_rules! endpoint {
    ($mod:ident, $rename:expr) => {
        pub mod $mod {
            use std::fmt;

            use serde_derive::Deserialize;

            #[derive(Deserialize, Debug)]
            pub struct Response {
                #[serde(rename = $rename)]
                pub series: Vec<super::Entry>,
            }

            impl fmt::Display for Response {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{:#?}", self)
                }
            }
        }
    };
}

endpoint!(calories, "activities-calories");
endpoint!(calories_bmr, "activities-caloriesBMR");
endpoint!(steps, "activities-steps");
endpoint!(distance, "activities-distance");
endpoint!(floors, "activities-floors");
endpoint!(elevation, "activities-elevation");
endpoint!(minutes_sedentary, "activities-minutesSedentary");
endpoint!(minutes_lightly_active, "activities-minutesLightlyActive");
endpoint!(minutes_fairly_active, "activities-minutesFairlyActive");
endpoint!(minutes_very_active, "activities-minutesVeryActive");
endpoint!(activity_calories, "activities-activityCalories");

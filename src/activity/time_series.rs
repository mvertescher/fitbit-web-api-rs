//! Get activity time series data.

use crate::{UserId, Period};

use chrono::naive::NaiveDate;
use serde_derive::Deserialize;
use strum_macros::*;
use url::Url;

#[derive(Debug, PartialEq, ToString)]
#[strum(serialize_all = "camel_case")]
pub enum Resource {
    Calories,
    CaloriesBMR,
    Steps,
    Distance,
    Floors,
    Elevation,
    Sedentary,
    LightlyActive,
    FairlyActive,
    VeryActive,
    ActivityCalories,
}

/// Generate the request URL from a resource.
pub fn url_from_date_and_period(user_id: UserId, resource: Resource, start: NaiveDate,
                                period: Period)
    -> Url
{
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
            use serde_derive::Deserialize;

            #[derive(Deserialize, Debug)]
            pub struct Response {
                #[serde(rename = $rename)]
                pub series: Vec<super::Entry>,
            }
        }
    };

    ($mod:ident) => {
        endpoint!($mod, stringify!(activities-$mod));
    };
}

endpoint!(calories);
endpoint!(calories_bmr, "activities-caloriesBMR");
endpoint!(steps);
endpoint!(distance);
endpoint!(floors);
endpoint!(elevation);
endpoint!(minutes_sedentary, "activities-minutesSedentary");
endpoint!(minutes_lightly_active, "activities-minutesLightlyActive");
endpoint!(minutes_fairly_active, "activities-minutesFairlyActive");
endpoint!(minutes_very_active, "activities-minutesVeryActive");
endpoint!(activity_calories, "activities-activityCalories");

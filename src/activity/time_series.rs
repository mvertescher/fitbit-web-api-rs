//! Get activity time series data.

use crate::{UserId, Period};

use chrono::naive::NaiveDate;
use serde_derive::Deserialize;
use url::Url;

// TODO: Enumerate tracker time series data
pub enum Resource {
    Calories,
    CaloriesBMR,
    Steps,
    Distance,
    Floors,
    Elevation,
    MinutesSedentary,
    MinutesLightlyActive,
    MinutesFairlyActive,
    MinutesVeryActive,
    ActivityCalories,
}

// TODO: Derive this somehow
impl ToString for Resource {
    fn to_string(&self) -> String {
        match self {
            Resource::Calories => "calories",
            Resource::CaloriesBMR => "caloriesBMR",
            Resource::Steps => "steps",
            Resource::Distance => "distance",
            Resource::Floors => "floors",
            Resource::Elevation => "elevation",
            Resource::MinutesSedentary => "minutesSedentary",
            Resource::MinutesLightlyActive => "minutesLightlyActive",
            Resource::MinutesFairlyActive => "minutesFairlyActive",
            Resource::MinutesVeryActive => "minutesVeryActive",
            Resource::ActivityCalories => "activityCalories",
        }.to_string()
    }
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

pub mod calories {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        #[serde(rename = "activities-calories")]
        pub series: Vec<super::Entry>,
    }
}

pub mod calories_bmr {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        #[serde(rename = "activities-caloriesBMR")]
        pub series: Vec<super::Entry>,
    }
}

pub mod steps {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        #[serde(rename = "activities-steps")]
        pub series: Vec<super::Entry>,
    }
}

pub mod distance {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        #[serde(rename = "activities-distance")]
        pub series: Vec<super::Entry>,
    }
}

pub mod floors {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        #[serde(rename = "activities-floors")]
        pub series: Vec<super::Entry>,
    }
}

pub mod elevation {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        #[serde(rename = "activities-elevation")]
        pub series: Vec<super::Entry>,
    }
}

pub mod minutes_sedentary {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        #[serde(rename = "activities-minutesSedentary")]
        pub series: Vec<super::Entry>,
    }
}

pub mod minutes_lightly_active {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        #[serde(rename = "activities-minutesLightlyActive")]
        pub series: Vec<super::Entry>,
    }
}

pub mod minutes_fairly_active {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        #[serde(rename = "activities-minutesFairlyActive")]
        pub series: Vec<super::Entry>,
    }
}

pub mod minutes_very_active {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        #[serde(rename = "activities-minutesVeryActive")]
        pub series: Vec<super::Entry>,
    }
}

pub mod activity_calories {
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Response {
        #[serde(rename = "activities-activityCalories")]
        pub series: Vec<super::Entry>,
    }
}

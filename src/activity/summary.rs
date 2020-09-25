//! Get daily activity summary.

use std::fmt;

use chrono::naive::NaiveDate;
use serde_derive::Deserialize;
use url::Url;

/// Generate the request URL from a date.
pub fn url_from_date(user_id: &str, date: NaiveDate) -> Url {
    Url::parse(&format!(
        "https://api.fitbit.com/1/user/{}/activities/date/{}.json",
        user_id, date,
    ))
    .unwrap()
}

/// Daily activity summary response.
#[derive(Deserialize, Debug)]
pub struct Response {
    pub activities: Vec<super::ActivityLog>,
    pub goals: Goals,
    pub summary: Summary,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[derive(Deserialize, Debug)]
pub struct Goals {
    #[serde(rename = "caloriesOut")]
    pub calories_out: usize,
    pub distance: f32,
    pub floors: usize,
    pub steps: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub activity_calories: usize,
    #[serde(rename = "caloriesBMR")]
    pub calories_bmr: usize,
    pub calories_out: usize,
    pub distances: Vec<Distance>,
    pub elevation: f32,
    pub fairly_active_minutes: usize,
    pub floors: usize,
    pub lightly_active_minutes: usize,
    pub marginal_calories: usize,
    pub sedentary_minutes: usize,
    pub steps: usize,
    pub very_active_minutes: usize,
}

#[derive(Deserialize, Debug)]
pub struct Distance {
    pub activity: String,
    pub distance: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        // An example activity data response
        let data = r#"
{
    "activities":[
        {
            "activityId":51007,
            "activityParentId":90019,
            "calories":230,
            "description":"7mph",
            "distance":2.04,
            "duration":1097053,
            "hasStartTime":true,
            "isFavorite":true,
            "logId":1154701,
            "name":"Treadmill, 0% Incline",
            "startTime":"00:25",
            "steps":3783
        }
    ],
    "goals":{
        "caloriesOut":2826,
        "distance":8.05,
        "floors":150,
        "steps":10000
     },
    "summary":{
        "activityCalories":230,
        "caloriesBMR":1913,
        "caloriesOut":2143,
        "distances":[
            {"activity":"tracker", "distance":1.32},
            {"activity":"loggedActivities", "distance":0},
            {"activity":"total","distance":1.32},
            {"activity":"veryActive", "distance":0.51},
            {"activity":"moderatelyActive", "distance":0.51},
            {"activity":"lightlyActive", "distance":0.51},
            {"activity":"sedentaryActive", "distance":0.51},
            {"activity":"Treadmill, 0% Incline", "distance":3.28}
        ],
        "elevation":48.77,
        "fairlyActiveMinutes":0,
        "floors":16,
        "lightlyActiveMinutes":0,
        "marginalCalories":200,
        "sedentaryMinutes":1166,
        "steps":0,
        "veryActiveMinutes":0
    }
}
"#;

        let _res: Response = serde_json::from_str(data).unwrap();
    }
}

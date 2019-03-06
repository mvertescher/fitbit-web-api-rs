//! Intraday heart rate time series data

use std::fmt;

use chrono::naive::{NaiveDate, NaiveTime};
use serde_derive::Deserialize;
use url::Url;

/// Build the URL for today.
pub fn url_from_today() -> Url {
    Url::parse("https://api.fitbit.com/1/user/-/activities/heart/date/today/1d/1sec.json").unwrap()
}

/// Build the URL for the current user and a given date range.
pub fn url_from_day(date: NaiveDate) -> Url {
    Url::parse(&format!("https://api.fitbit.com/1/user/-/activities/heart/date/{}/1d/1sec.json",
                        date)).unwrap()
}

/// Build the URL for the current user and a given date range.
pub fn url_from_date_range(start: NaiveDate, end: NaiveDate) -> Url {
    Url::parse(&format!("https://api.fitbit.com/1/user/-/activities/heart/date/{}/{}/1sec.json",
                        start, end)).unwrap()
}

/// Heart rate zones time series response.
#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(rename = "activities-heart")]
    pub daily_series: Option<Vec<DayEntry>>,
    #[serde(rename = "activities-heart-intraday")]
    pub intraday: Intraday,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DayEntry {
    pub date_time: NaiveDate,
    pub value: Value,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub custom_heart_rate_zones: Vec<super::HeartRateZone>,
    pub heart_rate_zones: Vec<super::HeartRateZone>,
    /// Resting heart rate
    pub value: Option<usize>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Intraday {
    pub dataset: Vec<IntradayEntry>,
    pub dataset_interval: usize,
    pub dataset_type: String,
}

#[derive(Deserialize, Debug)]
pub struct IntradayEntry {
    pub time: NaiveTime,
    pub value: usize,
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn deserialize() {
        // TODO: invalid characters below...
        let _data = r#"
{
    "activities-heart": [
        {
            "customHeartRateZones": [],
            "dateTime": "today",
            "heartRateZones": [
                {
                    "caloriesOut": 2.3246,
                    "max": 94,
                    "min": 30,
                    "minutes": 2,
                    "name": "Out of Range"
                },
                {
                    "caloriesOut": 0,
                    "max": 132,
                    "min": 94,
                    "minutes": 0,
                    "name": "Fat Burn"
                },
                {
                    "caloriesOut": 0,
                    "max": 160,
                    "min": 132,
                    "minutes": 0,
                    "name": "Cardio"
                },
                {
                    "caloriesOut": 0,
                    "max": 220,
                    "min": 160,
                    "minutes": 0,
                    "name": "Peak"
                }
            ],
            "value": "64.2"
        }
    ],
    "activities-heart-intraday": {
        "dataset": [
            {
                "time": "00:00:00",
                "value": 64
            },
            {
                "time": "00:00:10",
                "value": 63
            },
            {
                "time": "00:00:20",
                "value": 64
            },
            {
                "time": "00:00:30",
                "value": 65
            },
            {
                "time": "00:00:45",
                "value": 65
            }
        ],
        "datasetInterval": 1,
        "datasetType": "second"
    }
}
        "#;

        // let _res: Response = serde_json::from_str(data).unwrap();
    }
}

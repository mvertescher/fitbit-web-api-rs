//! Get sleep logs for a user.
//!
//! [More information?](https//dev.fitbit.com/build/reference/web-api/sleep/#get-sleep-logs)

use chrono::naive::NaiveDate;
use serde_derive::Deserialize;
use url::Url;

/// Generate the request URL from a single date.
pub fn url_from_date(user_id: &str, date: NaiveDate) -> Url {
    Url::parse(&format!(
        "https://api.fitbit.com/1.2/user/{}/sleep/date/{}.json",
        user_id, date
    ))
    .unwrap()
}

/// Generate the request URL from a start date and end date.
pub fn url_from_date_range(user_id: &str, start: NaiveDate, end: NaiveDate) -> Url {
    Url::parse(&format!(
        "https://api.fitbit.com/1.2/user/{}/sleep/date/{}/{}.json",
        user_id, start, end
    ))
    .unwrap()
}

/// Get sleep logs response.
#[derive(Deserialize, Debug)]
pub struct Response {
    pub sleep: Vec<SleepEntry>,
    pub summary: Option<Summary>,
}

/// A sleep entry for a particular day.
///
/// There can be multiple entries per day.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SleepEntry {}

/// A sleep summary.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub total_minutes_asleep: usize,
    pub total_sleep_records: usize,
    pub total_time_in_bed: usize,
}

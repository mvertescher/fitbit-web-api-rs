//! Get sleep list for a user.

use std::fmt::Display;

use crate::UserId;

use url::Url;

/// Generate the request URL from a user id.
pub fn url<T>(user_id: UserId, limit: u64, offset: u64, after_date: chrono::DateTime<T>) -> Url
where
    T: chrono::TimeZone,
    T::Offset: Display,
{
    Url::parse(&format!(
            "https://api.fitbit.com/1.2/user/{user_id}/sleep/list.json?limit={limit}&offset={offset}&sort=asc&afterDate={after_date}",
            user_id=user_id.to_string(),
            limit=limit,
            offset=offset,
            after_date=after_date.format("%Y-%m-%dT%H:%M:%S"),
        ))
        .unwrap()
}

pub mod get {

    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    #[serde(rename_all="camelCase")]
    pub struct Response {
        pagination: Pagination,
        sleep: Vec<Sleep>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all="camelCase")]
    pub struct Pagination {
        before_date: Option<String>,
        after_date: Option<String>,
        limit: u64,
        next: String,
        offset: u64,
        previous: String,
        sort: String,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all="camelCase")]
    pub struct Sleep {
        date_of_sleep: String,
        #[serde(rename="duration")]
        duration_millis: u64,
        efficiency: u64,
        end_time: String,
        info_code: u64,
        is_main_sleep: bool,
        levels: Levels,
        log_id: u64,
        minutes_after_wakeup: u64,
        minutes_asleep: u64,
        minutes_awake: u64,
        minutes_to_fall_asleep: u64,
        start_time: String,
        time_in_bed: u64,
        #[serde(rename="type")]
        type_: String,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all="camelCase")]
    pub struct Levels {
        summary: LevelSummary,
        data: Vec<SleepDataPoint>,
        short_data: Vec<SleepDataPoint>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct LevelSummary {
        deep: Summary,
        light: Summary,
        rem: Summary,
        wake: Summary,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Summary {
        count: u64,
        minutes: u64,
        thirty_day_avg_minutes: u64,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SleepDataPoint {
        date_time: String,
        level: SleepLevel,
        seconds: u64,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub enum SleepLevel {
        Wake,
        Light,
        Rem,
        Deep,
    }
}

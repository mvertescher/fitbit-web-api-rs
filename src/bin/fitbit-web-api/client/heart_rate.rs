//! Client for the heart rate endpoints

use super::get;

use fitbit_web_api::*;

pub(crate) fn get_time_series() {
    let period = heart_rate::time_series::Period::OneDay;
    let date = chrono::Local::today().naive_local();
    let url = heart_rate::time_series::url_from_date_and_period(&UserId::Current, date, period);
    let body = get(url);
    let ts: heart_rate::time_series::Response = serde_json::from_str(&body).unwrap();
    println!("{}", ts);
}

pub(crate) fn get_intraday_time_series() {
    let today = chrono::Local::today().naive_local();
    let url = heart_rate::intraday_time_series::url_from_day(&UserId::Current, today);
    let body = get(url);
    let ts: heart_rate::intraday_time_series::Response = serde_json::from_str(&body).unwrap();
    println!("{}", ts);
}

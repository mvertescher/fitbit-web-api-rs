//! Client for the activity endpoints

use super::get;

use fitbit_web_api::*;

pub(crate) fn get_goals() {
    let url = activity::goals::url();
    let body = get(url);
    let response: activity::goals::Response = serde_json::from_str(&body).unwrap();
    println!("{}", response);
}

pub(crate) fn get_lifetime_stats() {
    let url = activity::lifetime_stats::url(UserId::Current);
    let body = get(url);
    let response: activity::lifetime_stats::Response = serde_json::from_str(&body).unwrap();
    println!("{}", response);
}

macro_rules! display_resource {
    ($resource:ident, $body:ident) => {{
        let response: activity::time_series::$resource::Response = serde_json::from_str(&$body).unwrap();
        println!("{}", response);
    }};
}

pub(crate) fn get_time_series(resource: activity::time_series::Resource) {
    use activity::time_series::Resource;
    let url = activity::time_series::url_from_date_and_period(UserId::Current, &resource,
            chrono::Local::today().naive_local(), Period::OneDay);
    let body = get(url);
    match resource {
        Resource::Calories => display_resource!(calories, body),
        Resource::CaloriesBMR =>  display_resource!(calories_bmr, body),
        Resource::Steps => display_resource!(steps, body),
        Resource::Distance => display_resource!(distance, body),
        Resource::Floors => display_resource!(floors, body),
        Resource::Elevation => display_resource!(elevation, body),
        Resource::Sedentary => display_resource!(minutes_sedentary, body),
        Resource::LightlyActive => display_resource!(minutes_lightly_active, body),
        Resource::FairlyActive => display_resource!(minutes_fairly_active, body),
        Resource::VeryActive => display_resource!(minutes_very_active, body),
        Resource::ActivityCalories => display_resource!(activity_calories, body),
    };
}

pub(crate) fn get_summary() {
    let url = activity::summary::url_from_date("-", chrono::Local::today().naive_local());
    let body = get(url);
    let response: activity::summary::Response = serde_json::from_str(&body).unwrap();
    println!("{}", response);
}

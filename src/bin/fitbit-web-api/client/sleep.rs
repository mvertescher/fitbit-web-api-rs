//! Client for the sleep endpoints

use super::get;

use fitbit_web_api::*;

pub(crate) fn get_goal() {
    let url = sleep::goals::url(UserId::Current);
    let body = get(url);
    let response: sleep::goals::get::Response = serde_json::from_str(&body).unwrap();
    println!("{}", response);
}

pub(crate) fn get_list() {
    let url = sleep::list::url(UserId::Current);
    let _body = get(url);
    // TODO: Parse response
}

pub(crate) fn get_log() {
    let date = chrono::Local::today().naive_local();
    let url = sleep::logs::url_from_date("-", date);
    let _body = get(url);
    // TODO: Parse response
}

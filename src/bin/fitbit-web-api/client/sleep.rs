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
    println!("TODO: Get sleep list");
}


pub(crate) fn get_log() {
    println!("TODO: Get sleep log");
}

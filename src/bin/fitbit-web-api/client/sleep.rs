//! Client for the sleep endpoints

use std::error::Error;

use super::get;

use chrono::prelude::*;
use fitbit_web_api::*;

pub(crate) async fn get_goal() {
    let url = sleep::goals::url(UserId::Current);
    let body = get(url).await;
    let response: sleep::goals::get::Response = serde_json::from_str(&body).unwrap();
    println!("{}", response);
}

pub(crate) async fn get_list() {
    let url = sleep::list::url(
        UserId::Current,
        100,
        0,
        Utc::now().with_timezone(&Local) - chrono::Duration::days(8),
    );
    let body = get(url).await;
    let response: sleep::list::get::Response = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            println!("Error at: {}", &body[e.column()-20 ..= e.column()+20]);
            return
        },
    };
    println!("{:#?}", response);
}

pub(crate) async fn get_log() {
    let date = chrono::Local::today().naive_local();
    let url = sleep::logs::url_from_date("-", date);
    let _body = get(url).await;
    // TODO: Parse response
}

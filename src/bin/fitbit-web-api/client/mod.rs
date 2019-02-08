//! Top level client API for communicating with Fitbit.

use std::io::Write;

use fitbit_web_api::*;
use log::*;

mod oauth;

// For now, just record the token to a file.
const TOKEN_FILE: &str = "auth_token";

fn write_auth_token(token: String) {
    let mut file = std::fs::File::create(TOKEN_FILE).unwrap();
    file.write_all(token.as_bytes()).unwrap();
}

fn read_auth_token() -> String {
    match std::fs::read_to_string(TOKEN_FILE) {
        Ok(token) => token,
        Err(e) => {
            eprintln!("Failed to read the auth token ({})\nHave you run the `auth` command?", e);
            std::process::exit(1);
        }
    }
}

fn get(url: url::Url) -> String {
    let token: String = read_auth_token();

    let client = reqwest::Client::new();
    info!("GET {:?}", url);

    let mut res = client.get(url).bearer_auth(token).send().unwrap();
    let status = res.status();
    assert!(status.is_success(), format!("Failure status code: {}", status));

    let body = res.text().unwrap();
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    info!("{:#?}", v);

    body
}

pub(super) fn get_activity_goals() {
    let url = activity::goals::url();
    let body = get(url);
    let response: activity::goals::Response = serde_json::from_str(&body).unwrap();
    println!("{}", response);
}

pub(super) fn get_activity_summary() {
    let url = activity::summary::url_from_date("-", chrono::Local::today().naive_local());
    let body = get(url);
    let response: activity::summary::Response = serde_json::from_str(&body).unwrap();
    println!("{}", response);
}

pub(super) fn get_auth_token(id: String, secret: String) {
    let token = oauth::get_token(id, secret);
    write_auth_token(token);
    println!("Success! OAuth2 token recorded to {}.", TOKEN_FILE);
}

pub(super) fn get_badges() {
    let url = user::badges::get::url();
    let body = get(url);
    let badges: user::badges::get::Response = serde_json::from_str(&body).unwrap();
    println!("{}", badges);
}

pub(super) fn get_devices() {
    let url = devices::get::url();
    let body = get(url);
    let devices: devices::get::Response = serde_json::from_str(&body).unwrap();
    println!("{}", devices);
}

pub(super) fn get_profile() {
    let url = user::profile::url();
    let body = get(url);
    let profile: user::profile::Response = serde_json::from_str(&body).unwrap();
    println!("{}", profile);
}

//! # Fitbit Web API
//!
//! This crate holds strongly typed definitions for several of the public Fitbit Web APIs.
//!
//! Here's a list of the supported APIs by group:
//!
//! ## Devices Endpoints
//!
//! - [Get Devices](devices/get/index.html)

#![deny(warnings)]

extern crate chrono;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate url_serde;

use serde_derive::Deserialize;

pub mod activity;
pub mod devices;
pub mod user;

/// A user identifier used across many endpoints.
pub enum UserId {
    /// The current user logged in.
    Current,
    /// A user that is not logged in.
    User(String),
}

impl ToString for UserId {
    fn to_string(&self) -> String {
        match self {
            UserId::Current => "-".to_string(),
            UserId::User(id) => id.clone(),
        }
    }
}

/// A Fitbit product.
#[derive(Deserialize, Debug)]
pub enum Device {
    Aria,
    #[serde(rename = "Charge 2")]
    Charge2,
    #[serde(rename = "Charge 3")]
    Charge3,
    #[serde(rename = "Charge HR")]
    ChargeHr,
    Ionic,
    MobileTrack,
    Surge,
    Versa,
    #[doc(hidden)]
    __Nonexhaustive,
}

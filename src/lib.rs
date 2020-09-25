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

use serde::Deserialize;

pub mod activity;
pub mod body;
pub mod devices;
pub mod heart_rate;
pub mod sleep;
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

/// Possible period ranges for heart rate data.
pub enum Period {
    OneDay,
    SevenDays,
    ThirtyDays,
    OneWeek,
    OneMonth,
}

impl ToString for Period {
    fn to_string(&self) -> String {
        match self {
            Period::OneDay => "1d".to_string(),
            Period::SevenDays => "7d".to_string(),
            Period::ThirtyDays => "30d".to_string(),
            Period::OneWeek => "1w".to_string(),
            Period::OneMonth => "1m".to_string(),
        }
    }
}

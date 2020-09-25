//! Get a user's badges.

use std::fmt;

use crate::UserId;

use chrono::naive::NaiveDate;
use serde::Deserialize;
use url::Url;

/// The URL for this endpoint.
pub fn url(user: UserId) -> Url {
    Url::parse(&format!(
        "https://api.fitbit.com/1/user/{}/badges.json",
        user.to_string()
    ))
    .unwrap()
}

/// Expected response.
#[derive(Deserialize, Debug)]
pub struct Response {
    pub badges: Vec<Badge>,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for badge in &self.badges {
            write!(f, "{}\n\n", badge)?;
        }
        Ok(())
    }
}

/// Information about a badge a user has obtained.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
    pub badge_type: String,
    pub catagory: Option<String>,
    pub date_time: NaiveDate,
    pub description: String,
    pub earned_message: Option<String>,
    pub encoded_id: String,
    pub image100px: Url,
    pub image125px: Url,
    pub image300px: Url,
    pub image50px: Url,
    pub image75px: Url,
    pub name: String,
    pub share_text: String,
    pub short_description: String,
    pub short_name: String,
    pub times_achieved: usize,
    pub value: i32,
}

impl fmt::Display for Badge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

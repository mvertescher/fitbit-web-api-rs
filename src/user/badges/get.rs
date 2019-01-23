//! Get a user's badges.

use chrono::naive::NaiveDate;
use serde_derive::Deserialize;
use url::Url;

/// The URL for this endpoint.
pub fn url() -> Url {
    Url::parse("https://api.fitbit.com/1/user/-/badges.json").unwrap()
}

/// Expected response.
#[derive(Deserialize, Debug)]
pub struct Response {
    pub badges: Vec<Badge>,
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
    #[serde(with = "url_serde")]
    pub image100px: Url,
    #[serde(with = "url_serde")]
    pub image125px: Url,
    #[serde(with = "url_serde")]
    pub image300px: Url,
    #[serde(with = "url_serde")]
    pub image50px: Url,
    #[serde(with = "url_serde")]
    pub image75px: Url,
    pub name: String,
    pub share_text: String,
    pub short_description: String,
    pub short_name: String,
    pub times_achieved: usize,
    pub value: i32,
}

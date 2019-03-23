//! Body time series API

use crate::UserId;

use chrono::naive::NaiveDate;
use strum_macros::*;
use url::Url;

/// Possible body data resources
#[derive(Debug, PartialEq, ToString)]
#[strum(serialize_all = "snake_case")]
pub enum Resource {
    Bmi,
    Fat,
    Weight,
}

/// Possible period ranges
#[derive(Debug, PartialEq, ToString)]
#[strum(serialize_all = "snake_case")]
pub enum Period {
    #[strum(serialize="1d")]
    OneDay,
    #[strum(serialize="7d")]
    SevenDays,
    #[strum(serialize="30d")]
    ThirtyDays,
    #[strum(serialize="1w")]
    OneWeek,
    #[strum(serialize="1m")]
    OneMonth,
    #[strum(serialize="3m")]
    ThreeMonths,
    #[strum(serialize="6m")]
    SixMonths,
    #[strum(serialize="1y")]
    OneYear,
    Max,
}

/// Generate the request URL from a resource, start date and period.
pub fn url_from_date_and_period(user_id: UserId, resource: &Resource, start: NaiveDate,
                                period: Period)
    -> Url
{
    Url::parse(&format!(
        "https://api.fitbit.com/1/user/{}/body/{}/date/{}/{}.json",
        &user_id.to_string(),
        &resource.to_string(),
        start,
        &period.to_string()
    ))
    .unwrap()
}

/// Generate the request URL from a start date and end date.
pub fn url_from_dates(user_id: UserId, resource: &Resource, start: NaiveDate,
                      end: NaiveDate)
    -> Url
{
    Url::parse(&format!(
        "https://api.fitbit.com/1/user/{}/body/{}/date/{}/{}.json",
        &user_id.to_string(),
        &resource.to_string(),
        start,
        end
    ))
    .unwrap()
}

//! Get lifetime statistics.

use std::fmt;

use crate::UserId;

use chrono::naive::NaiveDate;
use serde_derive::Deserialize;
use url::Url;

pub fn url(user_id: UserId) -> Url {
    Url::parse(&format!(
        "https://api.fitbit.com/1/user/{}/activities.json",
        &user_id.to_string(),
    ))
    .unwrap()
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub best: Option<BestStats>,
    pub lifetime: LifetimeStats,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[derive(Deserialize, Debug)]
pub struct BestStats {
    pub total: BestStatsSet,
    pub tracker: BestStatsSet,
}

#[derive(Deserialize, Debug)]
pub struct BestStatsSet {
    pub distance: BestStat,
    pub floors: BestStat,
    pub steps: BestStat,
}

#[derive(Deserialize, Debug)]
pub struct BestStat {
    pub date: NaiveDate,
    pub value: f32,
}

#[derive(Deserialize, Debug)]
pub struct LifetimeStats {
    pub total: LifetimeStatsSet,
    pub tracker: LifetimeStatsSet,
}

#[derive(Deserialize, Debug)]
pub struct LifetimeStatsSet {
    pub distance: f32,
    pub floors: usize,
    pub steps: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
    fn deserialize() {
		let data = r#"
{
    "best":{
        "total":{
            "distance":{
                "date":"2012-01-07",
                "value":20.31597
            },
            "floors":{
                "date":"2012-01-29",
                "value":14
            },
            "steps":{
                "date":"2012-01-07",
                "value":26901
            }
        },
        "tracker":{
            "distance":{
                "date":"2012-01-07",
                "value":20.31597
            },
            "floors":  {
                "date":"2012-01-29",
                "value":14
            },
            "steps":{
                "date":"2012-01-07",
                "value":26901
            }
        }
    },
    "lifetime":{
        "total":{
            "distance":2711.62,
            "floors":2500,
            "steps":203300
        },
        "tracker":{
            "distance":2579.82,
            "floors":2500,
            "steps":106934
        }
    }
}
		"#;

        let _res: Response = serde_json::from_str(data).unwrap();
	}
}

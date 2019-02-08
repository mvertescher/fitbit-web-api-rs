//! Daily or weekly activity goals.

use std::fmt;

use serde_derive::{Deserialize, Serialize};
use url::Url;

/// Generate a goals endpoint URL for the current user's daily goals
pub fn url() -> Url {
    Url::parse("https://api.fitbit.com/1/user/-/activities/goals/daily.json").unwrap()
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub goals: Goals,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub goals: Goals,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Goals {
    pub calories_out: usize,
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
    "goals":{
        "caloriesOut": 2500,
        "distance": 8.05,
        "floors": 10,
        "steps": 10000
    }
}
        "#;

        let _res: Response = serde_json::from_str(data).unwrap();
    }
}

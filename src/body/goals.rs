//! Body weight and fat goals

use chrono::naive::NaiveDate;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WeightGoalContainer {
    pub goal: WeightGoal,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightGoal {
    pub start_date: NaiveDate,
    pub start_weight: String,
    pub weight: String,
}

#[derive(Serialize, Deserialize)]
pub struct FatGoalContainer {
    pub goal: FatGoal,
}

#[derive(Serialize, Deserialize)]
pub struct FatGoal {
    pub fat: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_weight_goal() {
        let data = r#"
{
    "goal":{
        "startDate": "2015-01-15",
        "startWeight": "150",
        "weight": "140"
    }
}
        "#;
        let _res: WeightGoalContainer = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn deserialize_fat_goal() {
        let data = r#"
{
    "goal":{
        "fat": "12"
    }
}
        "#;
        let _res: FatGoalContainer = serde_json::from_str(data).unwrap();
    }
}

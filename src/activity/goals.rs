//! Daily or weekly activity goals.

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub goals: Goals,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub goals: Goals,
}

#[derive(Serialize, Deserialize)]
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

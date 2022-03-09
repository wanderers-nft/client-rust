/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.123
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NullPlan {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "free")]
    Free,
    #[serde(rename = "start_up_monthly")]
    StartUpMonthly,
    #[serde(rename = "start_up_yearly")]
    StartUpYearly,
    #[serde(rename = "custom")]
    Custom,

}

impl ToString for NullPlan {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("unknown"),
            Self::Free => String::from("free"),
            Self::StartUpMonthly => String::from("start_up_monthly"),
            Self::StartUpYearly => String::from("start_up_yearly"),
            Self::Custom => String::from("custom"),
        }
    }
}





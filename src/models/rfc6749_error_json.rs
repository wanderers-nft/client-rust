/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.2.3
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rfc6749ErrorJson {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "error_debug", skip_serializing_if = "Option::is_none")]
    pub error_debug: Option<String>,
    #[serde(rename = "error_description", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(rename = "error_hint", skip_serializing_if = "Option::is_none")]
    pub error_hint: Option<String>,
    #[serde(rename = "status_code", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

impl Default for Rfc6749ErrorJson {
    fn default() -> Self {
        Self::new()
    }
}

impl Rfc6749ErrorJson {
    pub fn new() -> Rfc6749ErrorJson {
        Rfc6749ErrorJson {
                error: None,
                error_debug: None,
                error_description: None,
                error_hint: None,
                status_code: None,
        }
    }
}



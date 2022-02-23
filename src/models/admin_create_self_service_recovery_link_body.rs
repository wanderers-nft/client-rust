/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.107
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminCreateSelfServiceRecoveryLinkBody {
    /// Link Expires In  The recovery link will expire at that point in time. Defaults to the configuration value of `selfservice.flows.recovery.request_lifespan`.
    #[serde(rename = "expires_in", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<String>,
    #[serde(rename = "identity_id")]
    pub identity_id: String,
}

impl AdminCreateSelfServiceRecoveryLinkBody {
    pub fn new(identity_id: String) -> AdminCreateSelfServiceRecoveryLinkBody {
        AdminCreateSelfServiceRecoveryLinkBody {
            expires_in: None,
            identity_id,
        }
    }
}



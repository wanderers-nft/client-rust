/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.1
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// ManagedIdentitySchemaValidationResult : Ory Identity Schema Validation Result



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedIdentitySchemaValidationResult {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "valid", skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}

impl Default for ManagedIdentitySchemaValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl ManagedIdentitySchemaValidationResult {
    /// Ory Identity Schema Validation Result
    pub fn new() -> ManagedIdentitySchemaValidationResult {
        ManagedIdentitySchemaValidationResult {
                message: None,
                valid: None,
        }
    }
}


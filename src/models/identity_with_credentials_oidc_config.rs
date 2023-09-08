/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.2.2
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityWithCredentialsOidcConfig {
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::IdentityWithCredentialsPasswordConfig>>,
    /// A list of OpenID Connect Providers
    #[serde(rename = "providers", skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<crate::models::IdentityWithCredentialsOidcConfigProvider>>,
}

impl Default for IdentityWithCredentialsOidcConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityWithCredentialsOidcConfig {
    pub fn new() -> IdentityWithCredentialsOidcConfig {
        IdentityWithCredentialsOidcConfig {
                config: None,
                providers: None,
        }
    }
}



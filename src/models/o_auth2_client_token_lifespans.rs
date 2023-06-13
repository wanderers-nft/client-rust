/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.36
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// OAuth2ClientTokenLifespans : Lifespans of different token types issued for this OAuth 2.0 Client.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuth2ClientTokenLifespans {
    #[serde(rename = "authorization_code_grant_access_token_lifespan", skip_serializing_if = "Option::is_none")]
    pub authorization_code_grant_access_token_lifespan: Option<String>,
    #[serde(rename = "authorization_code_grant_id_token_lifespan", skip_serializing_if = "Option::is_none")]
    pub authorization_code_grant_id_token_lifespan: Option<String>,
    #[serde(rename = "authorization_code_grant_refresh_token_lifespan", skip_serializing_if = "Option::is_none")]
    pub authorization_code_grant_refresh_token_lifespan: Option<String>,
    #[serde(rename = "client_credentials_grant_access_token_lifespan", skip_serializing_if = "Option::is_none")]
    pub client_credentials_grant_access_token_lifespan: Option<String>,
    #[serde(rename = "implicit_grant_access_token_lifespan", skip_serializing_if = "Option::is_none")]
    pub implicit_grant_access_token_lifespan: Option<String>,
    #[serde(rename = "implicit_grant_id_token_lifespan", skip_serializing_if = "Option::is_none")]
    pub implicit_grant_id_token_lifespan: Option<String>,
    #[serde(rename = "jwt_bearer_grant_access_token_lifespan", skip_serializing_if = "Option::is_none")]
    pub jwt_bearer_grant_access_token_lifespan: Option<String>,
    #[serde(rename = "refresh_token_grant_access_token_lifespan", skip_serializing_if = "Option::is_none")]
    pub refresh_token_grant_access_token_lifespan: Option<String>,
    #[serde(rename = "refresh_token_grant_id_token_lifespan", skip_serializing_if = "Option::is_none")]
    pub refresh_token_grant_id_token_lifespan: Option<String>,
    #[serde(rename = "refresh_token_grant_refresh_token_lifespan", skip_serializing_if = "Option::is_none")]
    pub refresh_token_grant_refresh_token_lifespan: Option<String>,
}

impl Default for OAuth2ClientTokenLifespans {
    fn default() -> Self {
        Self::new()
    }
}

impl OAuth2ClientTokenLifespans {
    /// Lifespans of different token types issued for this OAuth 2.0 Client.
    pub fn new() -> OAuth2ClientTokenLifespans {
        OAuth2ClientTokenLifespans {
                authorization_code_grant_access_token_lifespan: None,
                authorization_code_grant_id_token_lifespan: None,
                authorization_code_grant_refresh_token_lifespan: None,
                client_credentials_grant_access_token_lifespan: None,
                implicit_grant_access_token_lifespan: None,
                implicit_grant_id_token_lifespan: None,
                jwt_bearer_grant_access_token_lifespan: None,
                refresh_token_grant_access_token_lifespan: None,
                refresh_token_grant_id_token_lifespan: None,
                refresh_token_grant_refresh_token_lifespan: None,
        }
    }
}



/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.30
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitSelfServiceLoginFlowWithWebAuthnMethodBody {
    /// Sending the anti-csrf token is only required for browser login flows.
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method should be set to \"webAuthn\" when logging in using the WebAuthn strategy.
    #[serde(rename = "method")]
    pub method: String,
    /// Login a WebAuthn Security Key  This must contain the ID of the WebAuthN connection.
    #[serde(rename = "webauthn_login", skip_serializing_if = "Option::is_none")]
    pub webauthn_login: Option<String>,
}

impl SubmitSelfServiceLoginFlowWithWebAuthnMethodBody {
    pub fn new(method: String) -> SubmitSelfServiceLoginFlowWithWebAuthnMethodBody {
        SubmitSelfServiceLoginFlowWithWebAuthnMethodBody {
            csrf_token: None,
            method,
            webauthn_login: None,
        }
    }
}


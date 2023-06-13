/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.36
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuth2LoginRequest {
    /// ID is the identifier (\"login challenge\") of the login request. It is used to identify the session.
    #[serde(rename = "challenge")]
    pub challenge: String,
    #[serde(rename = "client")]
    pub client: Box<crate::models::OAuth2Client>,
    #[serde(rename = "oidc_context", skip_serializing_if = "Option::is_none")]
    pub oidc_context: Option<Box<crate::models::OAuth2ConsentRequestOpenIdConnectContext>>,
    /// RequestURL is the original OAuth 2.0 Authorization URL requested by the OAuth 2.0 client. It is the URL which initiates the OAuth 2.0 Authorization Code or OAuth 2.0 Implicit flow. This URL is typically not needed, but might come in handy if you want to deal with additional request parameters.
    #[serde(rename = "request_url")]
    pub request_url: String,
    #[serde(rename = "requested_access_token_audience")]
    pub requested_access_token_audience: Vec<String>,
    #[serde(rename = "requested_scope")]
    pub requested_scope: Vec<String>,
    /// SessionID is the login session ID. If the user-agent reuses a login session (via cookie / remember flag) this ID will remain the same. If the user-agent did not have an existing authentication session (e.g. remember is false) this will be a new random value. This value is used as the \"sid\" parameter in the ID Token and in OIDC Front-/Back- channel logout. It's value can generally be used to associate consecutive login requests by a certain user.
    #[serde(rename = "session_id", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Skip, if true, implies that the client has requested the same scopes from the same user previously. If true, you can skip asking the user to grant the requested scopes, and simply forward the user to the redirect URL.  This feature allows you to update / set session information.
    #[serde(rename = "skip")]
    pub skip: bool,
    /// Subject is the user ID of the end-user that authenticated. Now, that end user needs to grant or deny the scope requested by the OAuth 2.0 client. If this value is set and `skip` is true, you MUST include this subject type when accepting the login request, or the request will fail.
    #[serde(rename = "subject")]
    pub subject: String,
}


impl OAuth2LoginRequest {
    pub fn new(challenge: String, client: crate::models::OAuth2Client, request_url: String, requested_access_token_audience: Vec<String>, requested_scope: Vec<String>, skip: bool, subject: String) -> OAuth2LoginRequest {
        OAuth2LoginRequest {
                challenge,
                client: Box::new(client),
                oidc_context: None,
                request_url,
                requested_access_token_audience,
                requested_scope,
                session_id: None,
                skip,
                subject,
        }
    }
}



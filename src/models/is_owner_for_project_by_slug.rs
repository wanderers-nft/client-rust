/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.123
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IsOwnerForProjectBySlug {
    /// ProjectSlug is the project's slug.
    #[serde(rename = "ProjectSlug")]
    pub project_slug: String,
    /// Subject is the subject from the API Token.
    #[serde(rename = "Subject")]
    pub subject: String,
}

impl IsOwnerForProjectBySlug {
    pub fn new(project_slug: String, subject: String) -> IsOwnerForProjectBySlug {
        IsOwnerForProjectBySlug {
            project_slug,
            subject,
        }
    }
}



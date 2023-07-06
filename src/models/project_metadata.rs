/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.41
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectMetadata {
    /// The Project's Creation Date
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "hosts")]
    pub hosts: Vec<String>,
    /// The project's ID.
    #[serde(rename = "id")]
    pub id: String,
    /// The project's name if set
    #[serde(rename = "name")]
    pub name: String,
    /// The project's slug
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// The state of the project. running Running halted Halted deleted Deleted
    #[serde(rename = "state")]
    pub state: StateEnum,
    #[serde(rename = "subscription_id", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "subscription_plan", skip_serializing_if = "Option::is_none")]
    pub subscription_plan: Option<String>,
    /// Last Time Project was Updated
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}


impl ProjectMetadata {
    pub fn new(created_at: String, hosts: Vec<String>, id: String, name: String, state: StateEnum, updated_at: String) -> ProjectMetadata {
        ProjectMetadata {
                created_at,
                hosts,
                id,
                name,
                slug: None,
                state,
                subscription_id: None,
                subscription_plan: None,
                updated_at,
        }
    }
}

/// The state of the project. running Running halted Halted deleted Deleted
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StateEnum {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "halted")]
    Halted,
    #[serde(rename = "deleted")]
    Deleted,
}


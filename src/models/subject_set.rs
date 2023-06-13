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
pub struct SubjectSet {
    /// Namespace of the Subject Set
    #[serde(rename = "namespace")]
    pub namespace: String,
    /// Object of the Subject Set
    #[serde(rename = "object")]
    pub object: String,
    /// Relation of the Subject Set
    #[serde(rename = "relation")]
    pub relation: String,
}


impl SubjectSet {
    pub fn new(namespace: String, object: String, relation: String) -> SubjectSet {
        SubjectSet {
                namespace,
                object,
                relation,
        }
    }
}



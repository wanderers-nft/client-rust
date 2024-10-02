/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.15.5
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// RelationshipPatch : Payload for patching a relationship
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipPatch {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionEnum>,
    #[serde(rename = "relation_tuple", skip_serializing_if = "Option::is_none")]
    pub relation_tuple: Option<Box<models::Relationship>>,
}

impl RelationshipPatch {
    /// Payload for patching a relationship
    pub fn new() -> RelationshipPatch {
        RelationshipPatch {
            action: None,
            relation_tuple: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionEnum {
    #[serde(rename = "insert")]
    Insert,
    #[serde(rename = "delete")]
    Delete,
}

impl Default for ActionEnum {
    fn default() -> ActionEnum {
        Self::Insert
    }
}


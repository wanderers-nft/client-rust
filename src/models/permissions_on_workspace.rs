/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.14.3
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// PermissionsOnWorkspace : Get Permissions on Project Request Parameters
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionsOnWorkspace {
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<std::collections::HashMap<String, bool>>,
}

impl PermissionsOnWorkspace {
    /// Get Permissions on Project Request Parameters
    pub fn new() -> PermissionsOnWorkspace {
        PermissionsOnWorkspace {
            permissions: None,
        }
    }
}


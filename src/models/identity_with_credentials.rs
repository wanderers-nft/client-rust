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

/// IdentityWithCredentials : Create Identity and Import Credentials
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityWithCredentials {
    #[serde(rename = "oidc", skip_serializing_if = "Option::is_none")]
    pub oidc: Option<Box<models::IdentityWithCredentialsOidc>>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<Box<models::IdentityWithCredentialsPassword>>,
}

impl IdentityWithCredentials {
    /// Create Identity and Import Credentials
    pub fn new() -> IdentityWithCredentials {
        IdentityWithCredentials {
            oidc: None,
            password: None,
        }
    }
}


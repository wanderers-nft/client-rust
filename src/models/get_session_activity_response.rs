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

/// GetSessionActivityResponse : Response of the getSessionActivity endpoint
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSessionActivityResponse {
    /// The list of data points.
    #[serde(rename = "data")]
    pub data: Vec<models::SessionActivityDatapoint>,
}

impl GetSessionActivityResponse {
    /// Response of the getSessionActivity endpoint
    pub fn new(data: Vec<models::SessionActivityDatapoint>) -> GetSessionActivityResponse {
        GetSessionActivityResponse {
            data,
        }
    }
}


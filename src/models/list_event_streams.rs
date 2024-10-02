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

/// ListEventStreams : Event Stream List
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListEventStreams {
    #[serde(rename = "event_streams", skip_serializing_if = "Option::is_none")]
    pub event_streams: Option<Vec<models::EventStream>>,
}

impl ListEventStreams {
    /// Event Stream List
    pub fn new() -> ListEventStreams {
        ListEventStreams {
            event_streams: None,
        }
    }
}


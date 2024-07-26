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

/// UiNodeMeta : This might include a label and other information that can optionally be used to render UIs.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UiNodeMeta {
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Box<models::UiText>>,
}

impl UiNodeMeta {
    /// This might include a label and other information that can optionally be used to render UIs.
    pub fn new() -> UiNodeMeta {
        UiNodeMeta {
            label: None,
        }
    }
}


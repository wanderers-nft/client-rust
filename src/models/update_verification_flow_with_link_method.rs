/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.15.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdateVerificationFlowWithLinkMethod : Update Verification Flow with Link Method
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateVerificationFlowWithLinkMethod {
    /// Sending the anti-csrf token is only required for browser login flows.
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Email to Verify  Needs to be set when initiating the flow. If the email is a registered verification email, a verification link will be sent. If the email is not known, a email with details on what happened will be sent instead.  format: email
    #[serde(rename = "email")]
    pub email: String,
    /// Method is the method that should be used for this verification flow  Allowed values are `link` and `code` link VerificationStrategyLink code VerificationStrategyCode
    #[serde(rename = "method")]
    pub method: MethodEnum,
    /// Transient data to pass along to any webhooks
    #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
    pub transient_payload: Option<serde_json::Value>,
}

impl UpdateVerificationFlowWithLinkMethod {
    /// Update Verification Flow with Link Method
    pub fn new(email: String, method: MethodEnum) -> UpdateVerificationFlowWithLinkMethod {
        UpdateVerificationFlowWithLinkMethod {
            csrf_token: None,
            email,
            method,
            transient_payload: None,
        }
    }
}
/// Method is the method that should be used for this verification flow  Allowed values are `link` and `code` link VerificationStrategyLink code VerificationStrategyCode
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MethodEnum {
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "code")]
    Code,
}

impl Default for MethodEnum {
    fn default() -> MethodEnum {
        Self::Link
    }
}


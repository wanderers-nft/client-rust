/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.14.4
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountExperienceConfiguration {
    #[serde(rename = "account_experience_theme_stylesheet", skip_serializing_if = "Option::is_none")]
    pub account_experience_theme_stylesheet: Option<String>,
    #[serde(rename = "favicon_type", skip_serializing_if = "Option::is_none")]
    pub favicon_type: Option<String>,
    #[serde(rename = "favicon_url", skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
    #[serde(rename = "kratos_selfservice_flows_recovery_enabled", skip_serializing_if = "Option::is_none")]
    pub kratos_selfservice_flows_recovery_enabled: Option<bool>,
    #[serde(rename = "kratos_selfservice_flows_registration_enabled", skip_serializing_if = "Option::is_none")]
    pub kratos_selfservice_flows_registration_enabled: Option<bool>,
    #[serde(rename = "kratos_selfservice_flows_verification_enabled", skip_serializing_if = "Option::is_none")]
    pub kratos_selfservice_flows_verification_enabled: Option<bool>,
    #[serde(rename = "logo_url", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "organization_map", skip_serializing_if = "Option::is_none")]
    pub organization_map: Option<std::collections::HashMap<String, String>>,
}

impl AccountExperienceConfiguration {
    pub fn new() -> AccountExperienceConfiguration {
        AccountExperienceConfiguration {
            account_experience_theme_stylesheet: None,
            favicon_type: None,
            favicon_url: None,
            kratos_selfservice_flows_recovery_enabled: None,
            kratos_selfservice_flows_registration_enabled: None,
            kratos_selfservice_flows_verification_enabled: None,
            logo_url: None,
            name: None,
            organization_map: None,
        }
    }
}

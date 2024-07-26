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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "cors_admin", skip_serializing_if = "Option::is_none")]
    pub cors_admin: Option<Box<models::ProjectCors>>,
    #[serde(rename = "cors_public", skip_serializing_if = "Option::is_none")]
    pub cors_public: Option<Box<models::ProjectCors>>,
    /// The environment of the project. prod Production stage Staging dev Development
    #[serde(rename = "environment")]
    pub environment: EnvironmentEnum,
    /// The project home region.  This is used to set where the project data is stored and where the project's endpoints are located. eu-central EUCentral us-east USEast us-west USWest us US global Global
    #[serde(rename = "home_region")]
    pub home_region: HomeRegionEnum,
    /// The project's ID.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the project.
    #[serde(rename = "name")]
    pub name: String,
    /// The configuration revision ID.
    #[serde(rename = "revision_id")]
    pub revision_id: String,
    #[serde(rename = "services")]
    pub services: Box<models::ProjectServices>,
    /// The project's slug
    #[serde(rename = "slug")]
    pub slug: String,
    /// The state of the project. running Running halted Halted deleted Deleted
    #[serde(rename = "state")]
    pub state: StateEnum,
    #[serde(rename = "workspace_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<Option<String>>,
}

impl Project {
    pub fn new(environment: EnvironmentEnum, home_region: HomeRegionEnum, id: String, name: String, revision_id: String, services: models::ProjectServices, slug: String, state: StateEnum) -> Project {
        Project {
            cors_admin: None,
            cors_public: None,
            environment,
            home_region,
            id,
            name,
            revision_id,
            services: Box::new(services),
            slug,
            state,
            workspace_id: None,
        }
    }
}
/// The environment of the project. prod Production stage Staging dev Development
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EnvironmentEnum {
    #[serde(rename = "prod")]
    Prod,
    #[serde(rename = "stage")]
    Stage,
    #[serde(rename = "dev")]
    Dev,
}

impl Default for EnvironmentEnum {
    fn default() -> EnvironmentEnum {
        Self::Prod
    }
}
/// The project home region.  This is used to set where the project data is stored and where the project's endpoints are located. eu-central EUCentral us-east USEast us-west USWest us US global Global
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HomeRegionEnum {
    #[serde(rename = "eu-central")]
    EuCentral,
    #[serde(rename = "us-east")]
    UsEast,
    #[serde(rename = "us-west")]
    UsWest,
    #[serde(rename = "us")]
    Us,
    #[serde(rename = "global")]
    Global,
}

impl Default for HomeRegionEnum {
    fn default() -> HomeRegionEnum {
        Self::EuCentral
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

impl Default for StateEnum {
    fn default() -> StateEnum {
        Self::Running
    }
}


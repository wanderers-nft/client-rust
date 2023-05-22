/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.33
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// CustomDomain : Custom Hostname



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomain {
    #[serde(rename = "cookie_domain", skip_serializing_if = "Option::is_none")]
    pub cookie_domain: Option<String>,
    #[serde(rename = "cors_allowed_origins", skip_serializing_if = "Option::is_none")]
    pub cors_allowed_origins: Option<Vec<String>>,
    #[serde(rename = "cors_enabled", skip_serializing_if = "Option::is_none")]
    pub cors_enabled: Option<bool>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "custom_ui_base_url", skip_serializing_if = "Option::is_none")]
    pub custom_ui_base_url: Option<String>,
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ssl_status", skip_serializing_if = "Option::is_none")]
    pub ssl_status: Option<SslStatusEnum>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "verification_errors", skip_serializing_if = "Option::is_none")]
    pub verification_errors: Option<Vec<String>>,
    #[serde(rename = "verification_status", skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
}

impl Default for CustomDomain {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomDomain {
    /// Custom Hostname
    pub fn new() -> CustomDomain {
        CustomDomain {
                cookie_domain: None,
                cors_allowed_origins: None,
                cors_enabled: None,
                created_at: None,
                custom_ui_base_url: None,
                hostname: None,
                id: None,
                ssl_status: None,
                updated_at: None,
                verification_errors: None,
                verification_status: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SslStatusEnum {
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "pending_validation")]
    PendingValidation,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "pending_issuance")]
    PendingIssuance,
    #[serde(rename = "pending_deployment")]
    PendingDeployment,
    #[serde(rename = "pending_deletion")]
    PendingDeletion,
    #[serde(rename = "pending_expiration")]
    PendingExpiration,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "initializing_timed_out")]
    InitializingTimedOut,
    #[serde(rename = "validation_timed_out")]
    ValidationTimedOut,
    #[serde(rename = "issuance_timed_out")]
    IssuanceTimedOut,
    #[serde(rename = "deployment_timed_out")]
    DeploymentTimedOut,
    #[serde(rename = "deletion_timed_out")]
    DeletionTimedOut,
    #[serde(rename = "pending_cleanup")]
    PendingCleanup,
    #[serde(rename = "staging_deployment")]
    StagingDeployment,
    #[serde(rename = "staging_active")]
    StagingActive,
    #[serde(rename = "deactivating")]
    Deactivating,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "backup_issued")]
    BackupIssued,
    #[serde(rename = "holding_deployment")]
    HoldingDeployment,
    #[serde(rename = "")]
    Empty,
}


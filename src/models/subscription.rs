/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.104
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The currently active plan of the subscription
    #[serde(rename = "current_plan")]
    pub current_plan: CurrentPlan,
    /// The ID of the stripe customer
    #[serde(rename = "customer_id")]
    pub customer_id: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ongoing_stripe_checkout_id", skip_serializing_if = "Option::is_none")]
    pub ongoing_stripe_checkout_id: Option<String>,
    /// Until when the subscription is payed
    #[serde(rename = "payed_until")]
    pub payed_until: String,
    #[serde(rename = "plan_changes_at", skip_serializing_if = "Option::is_none")]
    pub plan_changes_at: Option<String>,
    #[serde(rename = "plan_changes_to")]
    pub plan_changes_to: crate::models::NullPlan,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl Subscription {
    pub fn new(created_at: String, current_plan: CurrentPlan, customer_id: String, id: String, payed_until: String, plan_changes_to: crate::models::NullPlan, status: String, updated_at: String) -> Subscription {
        Subscription {
            created_at,
            current_plan,
            customer_id,
            id,
            ongoing_stripe_checkout_id: None,
            payed_until,
            plan_changes_at: None,
            plan_changes_to,
            status,
            updated_at,
        }
    }
}

/// The currently active plan of the subscription
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CurrentPlan {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "free")]
    Free,
    #[serde(rename = "start_up_monthly")]
    StartUpMonthly,
    #[serde(rename = "start_up_yearly")]
    StartUpYearly,
    #[serde(rename = "custom")]
    Custom,
}


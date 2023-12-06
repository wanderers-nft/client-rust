/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.4
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The currency of the subscription. To change this, a new subscription must be created. usd USD eur Euro
    #[serde(rename = "currency")]
    pub currency: CurrencyEnum,
    /// The currently active interval of the subscription monthly Monthly yearly Yearly
    #[serde(rename = "current_interval")]
    pub current_interval: CurrentIntervalEnum,
    /// The currently active plan of the subscription
    #[serde(rename = "current_plan")]
    pub current_plan: String,
    /// The ID of the stripe customer
    #[serde(rename = "customer_id")]
    pub customer_id: String,
    /// The ID of the subscription
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "interval_changes_to")]
    pub interval_changes_to: Option<String>,
    #[serde(rename = "ongoing_stripe_checkout_id", skip_serializing_if = "Option::is_none")]
    pub ongoing_stripe_checkout_id: Option<String>,
    /// Until when the subscription is payed
    #[serde(rename = "payed_until")]
    pub payed_until: String,
    #[serde(rename = "plan_changes_at", skip_serializing_if = "Option::is_none")]
    pub plan_changes_at: Option<String>,
    #[serde(rename = "plan_changes_to")]
    pub plan_changes_to: Option<String>,
    /// For `collection_method=charge_automatically` a subscription moves into `incomplete` if the initial payment attempt fails. A subscription in this state can only have metadata and default_source updated. Once the first invoice is paid, the subscription moves into an `active` state. If the first invoice is not paid within 23 hours, the subscription transitions to `incomplete_expired`. This is a terminal state, the open invoice will be voided and no further invoices will be generated.  A subscription that is currently in a trial period is `trialing` and moves to `active` when the trial period is over.  If subscription `collection_method=charge_automatically` it becomes `past_due` when payment to renew it fails and `canceled` or `unpaid` (depending on your subscriptions settings) when Stripe has exhausted all payment retry attempts.  If subscription `collection_method=send_invoice` it becomes `past_due` when its invoice is not paid by the due date, and `canceled` or `unpaid` if it is still not paid by an additional deadline after that. Note that when a subscription has a status of `unpaid`, no subsequent invoices will be attempted (invoices will be created, but then immediately automatically closed). After receiving updated payment information from a customer, you may choose to reopen and pay their closed invoices.
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}


impl Subscription {
    pub fn new(created_at: String, currency: CurrencyEnum, current_interval: CurrentIntervalEnum, current_plan: String, customer_id: String, id: String, interval_changes_to: Option<String>, payed_until: String, plan_changes_to: Option<String>, status: String, updated_at: String) -> Subscription {
        Subscription {
                created_at,
                currency,
                current_interval,
                current_plan,
                customer_id,
                id,
                interval_changes_to,
                ongoing_stripe_checkout_id: None,
                payed_until,
                plan_changes_at: None,
                plan_changes_to,
                status,
                updated_at,
        }
    }
}

/// The currency of the subscription. To change this, a new subscription must be created. usd USD eur Euro
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CurrencyEnum {
    #[serde(rename = "usd")]
    Usd,
    #[serde(rename = "eur")]
    Eur,
}
/// The currently active interval of the subscription monthly Monthly yearly Yearly
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CurrentIntervalEnum {
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "yearly")]
    Yearly,
}


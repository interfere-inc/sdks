pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateBillingCheckoutRequest {
    #[serde(rename = "billingInterval")]
    pub billing_interval: CreateBillingCheckoutRequestBillingInterval,
    #[serde(rename = "cancelUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    pub intent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<CreateBillingCheckoutRequestPlan>,
    #[serde(rename = "successUrl")]
    #[serde(default)]
    pub success_url: String,
}

impl CreateBillingCheckoutRequest {
    pub fn builder() -> CreateBillingCheckoutRequestBuilder {
        <CreateBillingCheckoutRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBillingCheckoutRequestBuilder {
    billing_interval: Option<CreateBillingCheckoutRequestBillingInterval>,
    cancel_url: Option<String>,
    intent_id: Option<String>,
    plan: Option<CreateBillingCheckoutRequestPlan>,
    success_url: Option<String>,
}

impl CreateBillingCheckoutRequestBuilder {
    pub fn billing_interval(mut self, value: CreateBillingCheckoutRequestBillingInterval) -> Self {
        self.billing_interval = Some(value);
        self
    }

    pub fn cancel_url(mut self, value: impl Into<String>) -> Self {
        self.cancel_url = Some(value.into());
        self
    }

    pub fn intent_id(mut self, value: impl Into<String>) -> Self {
        self.intent_id = Some(value.into());
        self
    }

    pub fn plan(mut self, value: CreateBillingCheckoutRequestPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn success_url(mut self, value: impl Into<String>) -> Self {
        self.success_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBillingCheckoutRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`billing_interval`](CreateBillingCheckoutRequestBuilder::billing_interval)
    /// - [`intent_id`](CreateBillingCheckoutRequestBuilder::intent_id)
    /// - [`success_url`](CreateBillingCheckoutRequestBuilder::success_url)
    pub fn build(self) -> Result<CreateBillingCheckoutRequest, BuildError> {
        Ok(CreateBillingCheckoutRequest {
            billing_interval: self.billing_interval.ok_or_else(|| BuildError::missing_field("billing_interval"))?,
            cancel_url: self.cancel_url,
            intent_id: self.intent_id.ok_or_else(|| BuildError::missing_field("intent_id"))?,
            plan: self.plan,
            success_url: self.success_url.ok_or_else(|| BuildError::missing_field("success_url"))?,
        })
    }
}


pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBillingPortalRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<CreateBillingPortalRequestFlow>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    pub intent_id: String,
    #[serde(rename = "returnUrl")]
    #[serde(default)]
    pub return_url: String,
}

impl CreateBillingPortalRequest {
    pub fn builder() -> CreateBillingPortalRequestBuilder {
        <CreateBillingPortalRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBillingPortalRequestBuilder {
    flow: Option<CreateBillingPortalRequestFlow>,
    intent_id: Option<String>,
    return_url: Option<String>,
}

impl CreateBillingPortalRequestBuilder {
    pub fn flow(mut self, value: CreateBillingPortalRequestFlow) -> Self {
        self.flow = Some(value);
        self
    }

    pub fn intent_id(mut self, value: impl Into<String>) -> Self {
        self.intent_id = Some(value.into());
        self
    }

    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBillingPortalRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`intent_id`](CreateBillingPortalRequestBuilder::intent_id)
    /// - [`return_url`](CreateBillingPortalRequestBuilder::return_url)
    pub fn build(self) -> Result<CreateBillingPortalRequest, BuildError> {
        Ok(CreateBillingPortalRequest {
            flow: self.flow,
            intent_id: self.intent_id.ok_or_else(|| BuildError::missing_field("intent_id"))?,
            return_url: self.return_url.ok_or_else(|| BuildError::missing_field("return_url"))?,
        })
    }
}


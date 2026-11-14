pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBillingCheckoutResponse {
    #[serde(default)]
    pub url: String,
}

impl CreateBillingCheckoutResponse {
    pub fn builder() -> CreateBillingCheckoutResponseBuilder {
        <CreateBillingCheckoutResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBillingCheckoutResponseBuilder {
    url: Option<String>,
}

impl CreateBillingCheckoutResponseBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBillingCheckoutResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](CreateBillingCheckoutResponseBuilder::url)
    pub fn build(self) -> Result<CreateBillingCheckoutResponse, BuildError> {
        Ok(CreateBillingCheckoutResponse {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}

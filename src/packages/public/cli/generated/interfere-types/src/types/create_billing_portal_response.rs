pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBillingPortalResponse {
    #[serde(default)]
    pub url: String,
}

impl CreateBillingPortalResponse {
    pub fn builder() -> CreateBillingPortalResponseBuilder {
        <CreateBillingPortalResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBillingPortalResponseBuilder {
    url: Option<String>,
}

impl CreateBillingPortalResponseBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBillingPortalResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](CreateBillingPortalResponseBuilder::url)
    pub fn build(self) -> Result<CreateBillingPortalResponse, BuildError> {
        Ok(CreateBillingPortalResponse {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}

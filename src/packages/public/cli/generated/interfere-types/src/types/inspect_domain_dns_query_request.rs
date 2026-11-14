pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for inspectDomainDns
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InspectDomainDnsQueryRequest {
    #[serde(default)]
    pub name: String,
}

impl InspectDomainDnsQueryRequest {
    pub fn builder() -> InspectDomainDnsQueryRequestBuilder {
        <InspectDomainDnsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InspectDomainDnsQueryRequestBuilder {
    name: Option<String>,
}

impl InspectDomainDnsQueryRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InspectDomainDnsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](InspectDomainDnsQueryRequestBuilder::name)
    pub fn build(self) -> Result<InspectDomainDnsQueryRequest, BuildError> {
        Ok(InspectDomainDnsQueryRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}


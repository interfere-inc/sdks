pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainConnectResponse {
    #[serde(rename = "applyUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_url: Option<String>,
    #[serde(default)]
    pub supported: bool,
}

impl DomainConnectResponse {
    pub fn builder() -> DomainConnectResponseBuilder {
        <DomainConnectResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainConnectResponseBuilder {
    apply_url: Option<String>,
    supported: Option<bool>,
}

impl DomainConnectResponseBuilder {
    pub fn apply_url(mut self, value: impl Into<String>) -> Self {
        self.apply_url = Some(value.into());
        self
    }

    pub fn supported(mut self, value: bool) -> Self {
        self.supported = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainConnectResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`supported`](DomainConnectResponseBuilder::supported)
    pub fn build(self) -> Result<DomainConnectResponse, BuildError> {
        Ok(DomainConnectResponse {
            apply_url: self.apply_url,
            supported: self.supported.ok_or_else(|| BuildError::missing_field("supported"))?,
        })
    }
}

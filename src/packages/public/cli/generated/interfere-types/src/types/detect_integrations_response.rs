pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DetectIntegrationsResponse {
    #[serde(default)]
    pub catalogue: Vec<DetectIntegrationsResponseCatalogueItem>,
    #[serde(default)]
    pub integrations: Vec<DetectIntegrationsResponseIntegrationsItem>,
    #[serde(default)]
    pub unsupported: Vec<DetectIntegrationsResponseUnsupportedItem>,
}

impl DetectIntegrationsResponse {
    pub fn builder() -> DetectIntegrationsResponseBuilder {
        <DetectIntegrationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DetectIntegrationsResponseBuilder {
    catalogue: Option<Vec<DetectIntegrationsResponseCatalogueItem>>,
    integrations: Option<Vec<DetectIntegrationsResponseIntegrationsItem>>,
    unsupported: Option<Vec<DetectIntegrationsResponseUnsupportedItem>>,
}

impl DetectIntegrationsResponseBuilder {
    pub fn catalogue(mut self, value: Vec<DetectIntegrationsResponseCatalogueItem>) -> Self {
        self.catalogue = Some(value);
        self
    }

    pub fn integrations(mut self, value: Vec<DetectIntegrationsResponseIntegrationsItem>) -> Self {
        self.integrations = Some(value);
        self
    }

    pub fn unsupported(mut self, value: Vec<DetectIntegrationsResponseUnsupportedItem>) -> Self {
        self.unsupported = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DetectIntegrationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`catalogue`](DetectIntegrationsResponseBuilder::catalogue)
    /// - [`integrations`](DetectIntegrationsResponseBuilder::integrations)
    /// - [`unsupported`](DetectIntegrationsResponseBuilder::unsupported)
    pub fn build(self) -> Result<DetectIntegrationsResponse, BuildError> {
        Ok(DetectIntegrationsResponse {
            catalogue: self.catalogue.ok_or_else(|| BuildError::missing_field("catalogue"))?,
            integrations: self.integrations.ok_or_else(|| BuildError::missing_field("integrations"))?,
            unsupported: self.unsupported.ok_or_else(|| BuildError::missing_field("unsupported"))?,
        })
    }
}

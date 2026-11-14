pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DetectIntegrationsResponseCatalogueItem {
    #[serde(default)]
    pub signal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<DetectIntegrationsResponseCatalogueItemProvider>,
    #[serde(default)]
    pub tool: String,
    #[serde(rename = "lookFor")]
    #[serde(default)]
    pub look_for: String,
}

impl DetectIntegrationsResponseCatalogueItem {
    pub fn builder() -> DetectIntegrationsResponseCatalogueItemBuilder {
        <DetectIntegrationsResponseCatalogueItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DetectIntegrationsResponseCatalogueItemBuilder {
    signal: Option<String>,
    provider: Option<DetectIntegrationsResponseCatalogueItemProvider>,
    tool: Option<String>,
    look_for: Option<String>,
}

impl DetectIntegrationsResponseCatalogueItemBuilder {
    pub fn signal(mut self, value: impl Into<String>) -> Self {
        self.signal = Some(value.into());
        self
    }

    pub fn provider(mut self, value: DetectIntegrationsResponseCatalogueItemProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn tool(mut self, value: impl Into<String>) -> Self {
        self.tool = Some(value.into());
        self
    }

    pub fn look_for(mut self, value: impl Into<String>) -> Self {
        self.look_for = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DetectIntegrationsResponseCatalogueItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`signal`](DetectIntegrationsResponseCatalogueItemBuilder::signal)
    /// - [`tool`](DetectIntegrationsResponseCatalogueItemBuilder::tool)
    /// - [`look_for`](DetectIntegrationsResponseCatalogueItemBuilder::look_for)
    pub fn build(self) -> Result<DetectIntegrationsResponseCatalogueItem, BuildError> {
        Ok(DetectIntegrationsResponseCatalogueItem {
            signal: self.signal.ok_or_else(|| BuildError::missing_field("signal"))?,
            provider: self.provider,
            tool: self.tool.ok_or_else(|| BuildError::missing_field("tool"))?,
            look_for: self.look_for.ok_or_else(|| BuildError::missing_field("look_for"))?,
        })
    }
}

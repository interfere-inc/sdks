pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DetectIntegrationsResponseUnsupportedItem {
    #[serde(default)]
    pub tool: String,
    #[serde(default)]
    pub evidence: Vec<String>,
}

impl DetectIntegrationsResponseUnsupportedItem {
    pub fn builder() -> DetectIntegrationsResponseUnsupportedItemBuilder {
        <DetectIntegrationsResponseUnsupportedItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DetectIntegrationsResponseUnsupportedItemBuilder {
    tool: Option<String>,
    evidence: Option<Vec<String>>,
}

impl DetectIntegrationsResponseUnsupportedItemBuilder {
    pub fn tool(mut self, value: impl Into<String>) -> Self {
        self.tool = Some(value.into());
        self
    }

    pub fn evidence(mut self, value: Vec<String>) -> Self {
        self.evidence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DetectIntegrationsResponseUnsupportedItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool`](DetectIntegrationsResponseUnsupportedItemBuilder::tool)
    /// - [`evidence`](DetectIntegrationsResponseUnsupportedItemBuilder::evidence)
    pub fn build(self) -> Result<DetectIntegrationsResponseUnsupportedItem, BuildError> {
        Ok(DetectIntegrationsResponseUnsupportedItem {
            tool: self.tool.ok_or_else(|| BuildError::missing_field("tool"))?,
            evidence: self.evidence.ok_or_else(|| BuildError::missing_field("evidence"))?,
        })
    }
}

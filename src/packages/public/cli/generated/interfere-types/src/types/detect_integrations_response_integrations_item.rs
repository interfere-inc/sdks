pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DetectIntegrationsResponseIntegrationsItem {
    pub provider: DetectIntegrationsResponseIntegrationsItemProvider,
    #[serde(default)]
    pub label: String,
    pub status: DetectIntegrationsResponseIntegrationsItemStatus,
    #[serde(rename = "likelyUsed")]
    #[serde(default)]
    pub likely_used: bool,
    #[serde(default)]
    pub evidence: Vec<String>,
    #[serde(rename = "nextStep")]
    #[serde(default)]
    pub next_step: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install: Option<DetectIntegrationsResponseIntegrationsItemInstall>,
}

impl DetectIntegrationsResponseIntegrationsItem {
    pub fn builder() -> DetectIntegrationsResponseIntegrationsItemBuilder {
        <DetectIntegrationsResponseIntegrationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DetectIntegrationsResponseIntegrationsItemBuilder {
    provider: Option<DetectIntegrationsResponseIntegrationsItemProvider>,
    label: Option<String>,
    status: Option<DetectIntegrationsResponseIntegrationsItemStatus>,
    likely_used: Option<bool>,
    evidence: Option<Vec<String>>,
    next_step: Option<String>,
    install: Option<DetectIntegrationsResponseIntegrationsItemInstall>,
}

impl DetectIntegrationsResponseIntegrationsItemBuilder {
    pub fn provider(mut self, value: DetectIntegrationsResponseIntegrationsItemProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn status(mut self, value: DetectIntegrationsResponseIntegrationsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn likely_used(mut self, value: bool) -> Self {
        self.likely_used = Some(value);
        self
    }

    pub fn evidence(mut self, value: Vec<String>) -> Self {
        self.evidence = Some(value);
        self
    }

    pub fn next_step(mut self, value: impl Into<String>) -> Self {
        self.next_step = Some(value.into());
        self
    }

    pub fn install(mut self, value: DetectIntegrationsResponseIntegrationsItemInstall) -> Self {
        self.install = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DetectIntegrationsResponseIntegrationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](DetectIntegrationsResponseIntegrationsItemBuilder::provider)
    /// - [`label`](DetectIntegrationsResponseIntegrationsItemBuilder::label)
    /// - [`status`](DetectIntegrationsResponseIntegrationsItemBuilder::status)
    /// - [`likely_used`](DetectIntegrationsResponseIntegrationsItemBuilder::likely_used)
    /// - [`evidence`](DetectIntegrationsResponseIntegrationsItemBuilder::evidence)
    /// - [`next_step`](DetectIntegrationsResponseIntegrationsItemBuilder::next_step)
    pub fn build(self) -> Result<DetectIntegrationsResponseIntegrationsItem, BuildError> {
        Ok(DetectIntegrationsResponseIntegrationsItem {
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            likely_used: self.likely_used.ok_or_else(|| BuildError::missing_field("likely_used"))?,
            evidence: self.evidence.ok_or_else(|| BuildError::missing_field("evidence"))?,
            next_step: self.next_step.ok_or_else(|| BuildError::missing_field("next_step"))?,
            install: self.install,
        })
    }
}

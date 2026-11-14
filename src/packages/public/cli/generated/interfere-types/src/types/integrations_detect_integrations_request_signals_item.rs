pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DetectIntegrationsRequestSignalsItem {
    /// A signal id from the catalogue this operation returns. Unknown ids are rejected.
    pub signal: DetectIntegrationsRequestSignalsItemSignal,
    /// Where it was found, such as a file path or variable name. Never include secret values.
    #[serde(default)]
    pub evidence: String,
}

impl DetectIntegrationsRequestSignalsItem {
    pub fn builder() -> DetectIntegrationsRequestSignalsItemBuilder {
        <DetectIntegrationsRequestSignalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DetectIntegrationsRequestSignalsItemBuilder {
    signal: Option<DetectIntegrationsRequestSignalsItemSignal>,
    evidence: Option<String>,
}

impl DetectIntegrationsRequestSignalsItemBuilder {
    pub fn signal(mut self, value: DetectIntegrationsRequestSignalsItemSignal) -> Self {
        self.signal = Some(value);
        self
    }

    pub fn evidence(mut self, value: impl Into<String>) -> Self {
        self.evidence = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DetectIntegrationsRequestSignalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`signal`](DetectIntegrationsRequestSignalsItemBuilder::signal)
    /// - [`evidence`](DetectIntegrationsRequestSignalsItemBuilder::evidence)
    pub fn build(self) -> Result<DetectIntegrationsRequestSignalsItem, BuildError> {
        Ok(DetectIntegrationsRequestSignalsItem {
            signal: self.signal.ok_or_else(|| BuildError::missing_field("signal"))?,
            evidence: self.evidence.ok_or_else(|| BuildError::missing_field("evidence"))?,
        })
    }
}

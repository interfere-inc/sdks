pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DetectIntegrationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signals: Option<Vec<DetectIntegrationsRequestSignalsItem>>,
}

impl DetectIntegrationsRequest {
    pub fn builder() -> DetectIntegrationsRequestBuilder {
        <DetectIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DetectIntegrationsRequestBuilder {
    signals: Option<Vec<DetectIntegrationsRequestSignalsItem>>,
}

impl DetectIntegrationsRequestBuilder {
    pub fn signals(mut self, value: Vec<DetectIntegrationsRequestSignalsItem>) -> Self {
        self.signals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DetectIntegrationsRequest`].
    pub fn build(self) -> Result<DetectIntegrationsRequest, BuildError> {
        Ok(DetectIntegrationsRequest {
            signals: self.signals,
        })
    }
}


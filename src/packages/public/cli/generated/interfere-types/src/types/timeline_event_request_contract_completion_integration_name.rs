pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractCompletionIntegrationName {
    pub kind: TimelineEventRequestContractCompletionIntegrationNameKind,
    #[serde(rename = "integrationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
}

impl TimelineEventRequestContractCompletionIntegrationName {
    pub fn builder() -> TimelineEventRequestContractCompletionIntegrationNameBuilder {
        <TimelineEventRequestContractCompletionIntegrationNameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractCompletionIntegrationNameBuilder {
    kind: Option<TimelineEventRequestContractCompletionIntegrationNameKind>,
    integration_name: Option<String>,
}

impl TimelineEventRequestContractCompletionIntegrationNameBuilder {
    pub fn kind(mut self, value: TimelineEventRequestContractCompletionIntegrationNameKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn integration_name(mut self, value: impl Into<String>) -> Self {
        self.integration_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractCompletionIntegrationName`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventRequestContractCompletionIntegrationNameBuilder::kind)
    pub fn build(self) -> Result<TimelineEventRequestContractCompletionIntegrationName, BuildError> {
        Ok(TimelineEventRequestContractCompletionIntegrationName {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            integration_name: self.integration_name,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContract {
    #[serde(default)]
    pub version: String,
    #[serde(rename = "requiredEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_evidence: Option<TimelineEventRequestContractRequiredEvidence>,
    #[serde(default)]
    pub scope: TimelineEventRequestContractScope,
    pub completion: TimelineEventRequestContractCompletion,
    pub resolution: TimelineEventRequestContractResolution,
}

impl TimelineEventRequestContract {
    pub fn builder() -> TimelineEventRequestContractBuilder {
        <TimelineEventRequestContractBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractBuilder {
    version: Option<String>,
    required_evidence: Option<TimelineEventRequestContractRequiredEvidence>,
    scope: Option<TimelineEventRequestContractScope>,
    completion: Option<TimelineEventRequestContractCompletion>,
    resolution: Option<TimelineEventRequestContractResolution>,
}

impl TimelineEventRequestContractBuilder {
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn required_evidence(mut self, value: TimelineEventRequestContractRequiredEvidence) -> Self {
        self.required_evidence = Some(value);
        self
    }

    pub fn scope(mut self, value: TimelineEventRequestContractScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn completion(mut self, value: TimelineEventRequestContractCompletion) -> Self {
        self.completion = Some(value);
        self
    }

    pub fn resolution(mut self, value: TimelineEventRequestContractResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContract`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](TimelineEventRequestContractBuilder::version)
    /// - [`scope`](TimelineEventRequestContractBuilder::scope)
    /// - [`completion`](TimelineEventRequestContractBuilder::completion)
    /// - [`resolution`](TimelineEventRequestContractBuilder::resolution)
    pub fn build(self) -> Result<TimelineEventRequestContract, BuildError> {
        Ok(TimelineEventRequestContract {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            required_evidence: self.required_evidence,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            completion: self.completion.ok_or_else(|| BuildError::missing_field("completion"))?,
            resolution: self.resolution.ok_or_else(|| BuildError::missing_field("resolution"))?,
        })
    }
}

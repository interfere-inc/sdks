pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractRequiredEvidence {
    pub kind: TimelineEventRequestContractRequiredEvidenceKind,
    #[serde(default)]
    pub provider: String,
    pub capability: TimelineEventRequestContractRequiredEvidenceCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: TimelineEventRequestContractRequiredEvidenceIncidentScope,
    pub dependency: TimelineEventRequestContractRequiredEvidenceDependency,
}

impl TimelineEventRequestContractRequiredEvidence {
    pub fn builder() -> TimelineEventRequestContractRequiredEvidenceBuilder {
        <TimelineEventRequestContractRequiredEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractRequiredEvidenceBuilder {
    kind: Option<TimelineEventRequestContractRequiredEvidenceKind>,
    provider: Option<String>,
    capability: Option<TimelineEventRequestContractRequiredEvidenceCapability>,
    incident_scope: Option<TimelineEventRequestContractRequiredEvidenceIncidentScope>,
    dependency: Option<TimelineEventRequestContractRequiredEvidenceDependency>,
}

impl TimelineEventRequestContractRequiredEvidenceBuilder {
    pub fn kind(mut self, value: TimelineEventRequestContractRequiredEvidenceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn capability(mut self, value: TimelineEventRequestContractRequiredEvidenceCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: TimelineEventRequestContractRequiredEvidenceIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    pub fn dependency(mut self, value: TimelineEventRequestContractRequiredEvidenceDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractRequiredEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventRequestContractRequiredEvidenceBuilder::kind)
    /// - [`provider`](TimelineEventRequestContractRequiredEvidenceBuilder::provider)
    /// - [`capability`](TimelineEventRequestContractRequiredEvidenceBuilder::capability)
    /// - [`incident_scope`](TimelineEventRequestContractRequiredEvidenceBuilder::incident_scope)
    /// - [`dependency`](TimelineEventRequestContractRequiredEvidenceBuilder::dependency)
    pub fn build(self) -> Result<TimelineEventRequestContractRequiredEvidence, BuildError> {
        Ok(TimelineEventRequestContractRequiredEvidence {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
        })
    }
}

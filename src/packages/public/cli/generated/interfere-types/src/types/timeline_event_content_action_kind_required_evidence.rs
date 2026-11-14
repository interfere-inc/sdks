pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentActionKindRequiredEvidence {
    pub kind: TimelineEventContentActionKindRequiredEvidenceKind,
    #[serde(default)]
    pub provider: String,
    pub capability: TimelineEventContentActionKindRequiredEvidenceCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: TimelineEventContentActionKindRequiredEvidenceIncidentScope,
    pub dependency: TimelineEventContentActionKindRequiredEvidenceDependency,
}

impl TimelineEventContentActionKindRequiredEvidence {
    pub fn builder() -> TimelineEventContentActionKindRequiredEvidenceBuilder {
        <TimelineEventContentActionKindRequiredEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindRequiredEvidenceBuilder {
    kind: Option<TimelineEventContentActionKindRequiredEvidenceKind>,
    provider: Option<String>,
    capability: Option<TimelineEventContentActionKindRequiredEvidenceCapability>,
    incident_scope: Option<TimelineEventContentActionKindRequiredEvidenceIncidentScope>,
    dependency: Option<TimelineEventContentActionKindRequiredEvidenceDependency>,
}

impl TimelineEventContentActionKindRequiredEvidenceBuilder {
    pub fn kind(mut self, value: TimelineEventContentActionKindRequiredEvidenceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn capability(mut self, value: TimelineEventContentActionKindRequiredEvidenceCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: TimelineEventContentActionKindRequiredEvidenceIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    pub fn dependency(mut self, value: TimelineEventContentActionKindRequiredEvidenceDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentActionKindRequiredEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventContentActionKindRequiredEvidenceBuilder::kind)
    /// - [`provider`](TimelineEventContentActionKindRequiredEvidenceBuilder::provider)
    /// - [`capability`](TimelineEventContentActionKindRequiredEvidenceBuilder::capability)
    /// - [`incident_scope`](TimelineEventContentActionKindRequiredEvidenceBuilder::incident_scope)
    /// - [`dependency`](TimelineEventContentActionKindRequiredEvidenceBuilder::dependency)
    pub fn build(self) -> Result<TimelineEventContentActionKindRequiredEvidence, BuildError> {
        Ok(TimelineEventContentActionKindRequiredEvidence {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidence {
    pub kind: TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKind,
    #[serde(default)]
    pub provider: String,
    pub capability: TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceIncidentScope,
    pub dependency: TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency,
}

impl TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidence {
    pub fn builder() -> TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder {
        <TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder {
    kind: Option<TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKind>,
    provider: Option<String>,
    capability: Option<TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceCapability>,
    incident_scope: Option<TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceIncidentScope>,
    dependency: Option<TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency>,
}

impl TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder {
    pub fn kind(mut self, value: TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn capability(mut self, value: TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    pub fn dependency(mut self, value: TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder::kind)
    /// - [`provider`](TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder::provider)
    /// - [`capability`](TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder::capability)
    /// - [`incident_scope`](TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder::incident_scope)
    /// - [`dependency`](TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder::dependency)
    pub fn build(self) -> Result<TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidence, BuildError> {
        Ok(TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidence {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
        })
    }
}

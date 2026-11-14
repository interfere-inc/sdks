pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidence {
    pub kind: TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKind,
    #[serde(default)]
    pub provider: String,
    pub capability: TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceIncidentScope,
    pub dependency: TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency,
}

impl TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidence {
    pub fn builder() -> TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder {
        <TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder {
    kind: Option<TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKind>,
    provider: Option<String>,
    capability: Option<TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceCapability>,
    incident_scope: Option<TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceIncidentScope>,
    dependency: Option<TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency>,
}

impl TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder {
    pub fn kind(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn capability(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    pub fn dependency(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder::kind)
    /// - [`provider`](TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder::provider)
    /// - [`capability`](TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder::capability)
    /// - [`incident_scope`](TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder::incident_scope)
    /// - [`dependency`](TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder::dependency)
    pub fn build(self) -> Result<TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidence, BuildError> {
        Ok(TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidence {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
        })
    }
}

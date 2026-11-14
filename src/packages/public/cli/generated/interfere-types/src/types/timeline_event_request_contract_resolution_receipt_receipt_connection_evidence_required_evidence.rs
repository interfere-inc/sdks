pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence {
    pub kind: TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKind,
    #[serde(default)]
    pub provider: String,
    pub capability: TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceIncidentScope,
    pub dependency: TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency,
}

impl TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence {
    pub fn builder() -> TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder {
        <TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder {
    kind: Option<TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKind>,
    provider: Option<String>,
    capability: Option<TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceCapability>,
    incident_scope: Option<TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceIncidentScope>,
    dependency: Option<TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency>,
}

impl TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder {
    pub fn kind(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn capability(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    pub fn dependency(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder::kind)
    /// - [`provider`](TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder::provider)
    /// - [`capability`](TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder::capability)
    /// - [`incident_scope`](TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder::incident_scope)
    /// - [`dependency`](TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder::dependency)
    pub fn build(self) -> Result<TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence, BuildError> {
        Ok(TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<serde_json::Value>,
    #[serde(default)]
    pub provider: String,
    pub capability: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceIncidentScope,
    #[serde(default)]
    pub dependency: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidence {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder {
    kind: Option<serde_json::Value>,
    provider: Option<String>,
    capability: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceCapability>,
    incident_scope: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceIncidentScope>,
    dependency: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder {
    pub fn kind(mut self, value: serde_json::Value) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn capability(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    pub fn dependency(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder::provider)
    /// - [`capability`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder::capability)
    /// - [`incident_scope`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder::incident_scope)
    /// - [`dependency`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceBuilder::dependency)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidence, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidence {
            kind: self.kind,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
        })
    }
}

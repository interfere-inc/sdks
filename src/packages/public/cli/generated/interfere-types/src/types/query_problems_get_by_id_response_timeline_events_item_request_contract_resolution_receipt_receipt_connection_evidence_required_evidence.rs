pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<serde_json::Value>,
    #[serde(default)]
    pub provider: String,
    pub capability: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceIncidentScope,
    #[serde(default)]
    pub dependency: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder {
    kind: Option<serde_json::Value>,
    provider: Option<String>,
    capability: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceCapability>,
    incident_scope: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceIncidentScope>,
    dependency: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder {
    pub fn kind(mut self, value: serde_json::Value) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn capability(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    pub fn dependency(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder::provider)
    /// - [`capability`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder::capability)
    /// - [`incident_scope`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder::incident_scope)
    /// - [`dependency`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceBuilder::dependency)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence {
            kind: self.kind,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
        })
    }
}

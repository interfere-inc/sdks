pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidence {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKind,
    #[serde(default)]
    pub provider: String,
    pub capability: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceIncidentScope,
    pub dependency: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependency,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidence {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKind>,
    provider: Option<String>,
    capability: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceCapability>,
    incident_scope: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceIncidentScope>,
    dependency: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependency>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn capability(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    pub fn dependency(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceBuilder::kind)
    /// - [`provider`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceBuilder::provider)
    /// - [`capability`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceBuilder::capability)
    /// - [`incident_scope`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceBuilder::incident_scope)
    /// - [`dependency`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceBuilder::dependency)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidence, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidence {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
        })
    }
}

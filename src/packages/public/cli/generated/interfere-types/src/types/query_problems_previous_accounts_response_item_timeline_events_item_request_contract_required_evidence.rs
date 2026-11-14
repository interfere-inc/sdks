pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidence {
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKind,
    #[serde(default)]
    pub provider: String,
    pub capability: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceIncidentScope,
    pub dependency: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependency,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidence {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceBuilder {
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKind>,
    provider: Option<String>,
    capability: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceCapability>,
    incident_scope: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceIncidentScope>,
    dependency: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependency>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceBuilder {
    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn capability(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    pub fn dependency(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceBuilder::kind)
    /// - [`provider`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceBuilder::provider)
    /// - [`capability`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceBuilder::capability)
    /// - [`incident_scope`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceBuilder::incident_scope)
    /// - [`dependency`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceBuilder::dependency)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidence, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidence {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
        })
    }
}

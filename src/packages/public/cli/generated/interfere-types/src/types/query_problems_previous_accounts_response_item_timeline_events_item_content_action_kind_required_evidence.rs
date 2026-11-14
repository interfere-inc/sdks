pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidence {
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKind,
    #[serde(default)]
    pub provider: String,
    pub capability: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceIncidentScope,
    pub dependency: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependency,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidence {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceBuilder {
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKind>,
    provider: Option<String>,
    capability: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceCapability>,
    incident_scope: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceIncidentScope>,
    dependency: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependency>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceBuilder {
    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn capability(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    pub fn dependency(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceBuilder::kind)
    /// - [`provider`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceBuilder::provider)
    /// - [`capability`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceBuilder::capability)
    /// - [`incident_scope`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceBuilder::incident_scope)
    /// - [`dependency`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceBuilder::dependency)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidence, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidence {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
        })
    }
}

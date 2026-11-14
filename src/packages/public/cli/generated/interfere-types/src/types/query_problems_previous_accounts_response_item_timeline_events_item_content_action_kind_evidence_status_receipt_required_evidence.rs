pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidence {
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKind,
    #[serde(default)]
    pub provider: String,
    pub capability: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceIncidentScope,
    pub dependency: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidence {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder {
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKind>,
    provider: Option<String>,
    capability: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceCapability>,
    incident_scope: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceIncidentScope>,
    dependency: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder {
    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn capability(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    pub fn dependency(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder::kind)
    /// - [`provider`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder::provider)
    /// - [`capability`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder::capability)
    /// - [`incident_scope`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder::incident_scope)
    /// - [`dependency`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder::dependency)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidence, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidence {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
        })
    }
}

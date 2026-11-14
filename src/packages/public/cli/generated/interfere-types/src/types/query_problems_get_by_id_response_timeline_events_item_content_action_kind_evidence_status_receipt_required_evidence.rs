pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidence {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKind,
    #[serde(default)]
    pub provider: String,
    pub capability: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceIncidentScope,
    pub dependency: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidence {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKind>,
    provider: Option<String>,
    capability: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceCapability>,
    incident_scope: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceIncidentScope>,
    dependency: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn capability(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    pub fn dependency(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder::kind)
    /// - [`provider`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder::provider)
    /// - [`capability`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder::capability)
    /// - [`incident_scope`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder::incident_scope)
    /// - [`dependency`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceBuilder::dependency)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidence, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidence {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
        })
    }
}

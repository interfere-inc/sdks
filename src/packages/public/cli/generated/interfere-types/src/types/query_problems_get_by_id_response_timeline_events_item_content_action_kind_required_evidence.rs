pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidence {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKind,
    #[serde(default)]
    pub provider: String,
    pub capability: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceIncidentScope,
    pub dependency: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependency,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidence {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKind>,
    provider: Option<String>,
    capability: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceCapability>,
    incident_scope: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceIncidentScope>,
    dependency: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependency>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn capability(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    pub fn dependency(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceBuilder::kind)
    /// - [`provider`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceBuilder::provider)
    /// - [`capability`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceBuilder::capability)
    /// - [`incident_scope`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceBuilder::incident_scope)
    /// - [`dependency`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceBuilder::dependency)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidence, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidence {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
        })
    }
}

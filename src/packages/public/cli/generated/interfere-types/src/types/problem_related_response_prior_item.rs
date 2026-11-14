pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemRelatedResponsePriorItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub evidence: ProblemRelatedResponsePriorItemEvidence,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "rootCause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_cause: Option<String>,
    pub status: ProblemRelatedResponsePriorItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ProblemRelatedResponsePriorItem {
    pub fn builder() -> ProblemRelatedResponsePriorItemBuilder {
        <ProblemRelatedResponsePriorItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemRelatedResponsePriorItemBuilder {
    description: Option<String>,
    evidence: Option<ProblemRelatedResponsePriorItemEvidence>,
    id: Option<String>,
    root_cause: Option<String>,
    status: Option<ProblemRelatedResponsePriorItemStatus>,
    title: Option<String>,
}

impl ProblemRelatedResponsePriorItemBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn evidence(mut self, value: ProblemRelatedResponsePriorItemEvidence) -> Self {
        self.evidence = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn root_cause(mut self, value: impl Into<String>) -> Self {
        self.root_cause = Some(value.into());
        self
    }

    pub fn status(mut self, value: ProblemRelatedResponsePriorItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemRelatedResponsePriorItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`evidence`](ProblemRelatedResponsePriorItemBuilder::evidence)
    /// - [`id`](ProblemRelatedResponsePriorItemBuilder::id)
    /// - [`status`](ProblemRelatedResponsePriorItemBuilder::status)
    pub fn build(self) -> Result<ProblemRelatedResponsePriorItem, BuildError> {
        Ok(ProblemRelatedResponsePriorItem {
            description: self.description,
            evidence: self.evidence.ok_or_else(|| BuildError::missing_field("evidence"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            root_cause: self.root_cause,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            title: self.title,
        })
    }
}

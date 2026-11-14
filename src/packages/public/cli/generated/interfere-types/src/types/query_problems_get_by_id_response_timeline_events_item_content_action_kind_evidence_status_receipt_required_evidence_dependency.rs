pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    pub mode: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyMode,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder {
    group_id: Option<String>,
    mode: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyMode>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder::group_id)
    /// - [`mode`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder::mode)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

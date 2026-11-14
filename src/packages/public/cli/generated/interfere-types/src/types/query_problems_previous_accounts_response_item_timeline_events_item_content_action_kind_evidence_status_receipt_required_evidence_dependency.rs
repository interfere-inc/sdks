pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    pub mode: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyMode,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder {
    group_id: Option<String>,
    mode: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyMode>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder::group_id)
    /// - [`mode`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder::mode)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

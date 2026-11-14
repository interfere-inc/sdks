pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependency {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    pub mode: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependencyMode,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependency {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependencyBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependencyBuilder {
    group_id: Option<String>,
    mode: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependencyMode>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependencyBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependencyMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependencyBuilder::group_id)
    /// - [`mode`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependencyBuilder::mode)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependency, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRequiredEvidenceDependency {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependency {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    pub mode: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependencyMode,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependency {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependencyBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependencyBuilder {
    group_id: Option<String>,
    mode: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependencyMode>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependencyBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependencyMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependencyBuilder::group_id)
    /// - [`mode`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependencyBuilder::mode)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependency, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractRequiredEvidenceDependency {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

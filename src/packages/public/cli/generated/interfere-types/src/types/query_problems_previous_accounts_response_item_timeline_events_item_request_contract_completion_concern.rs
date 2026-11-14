pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcern {
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcernKind,
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    pub concern: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcernConcern,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcern {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcernBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcernBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcernBuilder {
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcernKind>,
    target_id: Option<String>,
    concern: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcernConcern>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcernBuilder {
    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcernKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn target_id(mut self, value: impl Into<String>) -> Self {
        self.target_id = Some(value.into());
        self
    }

    pub fn concern(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcernConcern) -> Self {
        self.concern = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcern`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcernBuilder::kind)
    /// - [`concern`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcernBuilder::concern)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcern, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionConcern {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            target_id: self.target_id,
            concern: self.concern.ok_or_else(|| BuildError::missing_field("concern"))?,
        })
    }
}

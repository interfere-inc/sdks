pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKind {
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindKind,
    #[serde(rename = "actionKind")]
    pub action_kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindActionKind,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKind {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindBuilder {
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindKind>,
    action_kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindActionKind>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindBuilder {
    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn action_kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindActionKind) -> Self {
        self.action_kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKind`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindBuilder::kind)
    /// - [`action_kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindBuilder::action_kind)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKind, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKind {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            action_kind: self.action_kind.ok_or_else(|| BuildError::missing_field("action_kind"))?,
        })
    }
}

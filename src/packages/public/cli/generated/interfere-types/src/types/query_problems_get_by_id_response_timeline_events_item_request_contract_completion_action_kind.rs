pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKind {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKindKind,
    #[serde(rename = "actionKind")]
    pub action_kind: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKindActionKind,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKind {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKindBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKindBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKindBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKindKind>,
    action_kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKindActionKind>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKindBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKindKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn action_kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKindActionKind) -> Self {
        self.action_kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKind`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKindBuilder::kind)
    /// - [`action_kind`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKindBuilder::action_kind)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKind, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionActionKind {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            action_kind: self.action_kind.ok_or_else(|| BuildError::missing_field("action_kind"))?,
        })
    }
}

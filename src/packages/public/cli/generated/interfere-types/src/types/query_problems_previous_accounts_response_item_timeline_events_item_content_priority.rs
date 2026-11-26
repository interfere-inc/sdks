pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriority {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriorityBasis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriorityPriority>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriority {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriorityBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriorityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriorityBuilder {
    basis: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriorityBasis>,
    priority: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriorityPriority>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriorityBuilder {
    pub fn basis(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriorityBasis) -> Self {
        self.basis = Some(value);
        self
    }

    pub fn priority(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriorityPriority) -> Self {
        self.priority = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriority`].
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriority, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPriority {
            basis: self.basis,
            priority: self.priority,
        })
    }
}

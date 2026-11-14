pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBasis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisPriority>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasis {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBuilder {
    basis: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBasis>,
    priority: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisPriority>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBuilder {
    pub fn basis(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBasis) -> Self {
        self.basis = Some(value);
        self
    }

    pub fn priority(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisPriority) -> Self {
        self.priority = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasis`].
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasis, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasis {
            basis: self.basis,
            priority: self.priority,
        })
    }
}

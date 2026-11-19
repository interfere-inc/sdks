pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemId {
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemIdKind,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemId {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemIdBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemIdBuilder {
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemIdKind>,
    problem_id: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemIdBuilder {
    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemIdKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemIdBuilder::kind)
    /// - [`problem_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemIdBuilder::problem_id)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemId, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalProblemId {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(default)]
    pub sequence: i64,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSharedAskBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSharedAskBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSharedAskBuilder {
    problem_id: Option<String>,
    sequence: Option<i64>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSharedAskBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSharedAskBuilder::problem_id)
    /// - [`sequence`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSharedAskBuilder::sequence)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            sequence: self.sequence.ok_or_else(|| BuildError::missing_field("sequence"))?,
        })
    }
}

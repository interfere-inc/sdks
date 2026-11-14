pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(default)]
    pub sequence: i64,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSharedAskBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSharedAskBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSharedAskBuilder {
    problem_id: Option<String>,
    sequence: Option<i64>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSharedAskBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSharedAskBuilder::problem_id)
    /// - [`sequence`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSharedAskBuilder::sequence)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            sequence: self.sequence.ok_or_else(|| BuildError::missing_field("sequence"))?,
        })
    }
}

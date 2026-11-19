pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemId {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemIdKind,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemId {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemIdBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemIdBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemIdKind>,
    problem_id: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemIdBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemIdKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemIdBuilder::kind)
    /// - [`problem_id`](QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemIdBuilder::problem_id)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemId, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalProblemId {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}

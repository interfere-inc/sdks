pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetry {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetryKind,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetry {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetryBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetryBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetryKind>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetryBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetryKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetryBuilder::kind)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetry, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentPreviousStatusRetry {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}

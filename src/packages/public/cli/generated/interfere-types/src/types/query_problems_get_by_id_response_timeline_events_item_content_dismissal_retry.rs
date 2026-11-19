pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetry {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetryKind,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetry {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetryBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetryBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetryKind>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetryBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetryKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetryBuilder::kind)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetry, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalRetry {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}

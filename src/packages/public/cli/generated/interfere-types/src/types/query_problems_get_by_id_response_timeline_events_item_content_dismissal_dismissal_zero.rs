pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalZero {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalZeroKind,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalZero {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalZeroBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalZeroBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalZeroKind>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalZeroBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalZeroKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalZeroBuilder::kind)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalZero, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentDismissalDismissalZero {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}

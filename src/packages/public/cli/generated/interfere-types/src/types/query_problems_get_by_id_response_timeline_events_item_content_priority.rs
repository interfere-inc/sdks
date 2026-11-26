pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentPriority {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentPriorityBasis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentPriorityPriority>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentPriority {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentPriorityBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentPriorityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentPriorityBuilder {
    basis: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentPriorityBasis>,
    priority: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentPriorityPriority>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentPriorityBuilder {
    pub fn basis(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentPriorityBasis) -> Self {
        self.basis = Some(value);
        self
    }

    pub fn priority(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentPriorityPriority) -> Self {
        self.priority = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentPriority`].
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentPriority, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentPriority {
            basis: self.basis,
            priority: self.priority,
        })
    }
}

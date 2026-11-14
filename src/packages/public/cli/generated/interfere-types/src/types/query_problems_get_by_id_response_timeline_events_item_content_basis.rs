pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentBasis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentBasisBasis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentBasisPriority>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentBasis {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentBasisBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentBasisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentBasisBuilder {
    basis: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentBasisBasis>,
    priority: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentBasisPriority>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentBasisBuilder {
    pub fn basis(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentBasisBasis) -> Self {
        self.basis = Some(value);
        self
    }

    pub fn priority(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentBasisPriority) -> Self {
        self.priority = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentBasis`].
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentBasis, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentBasis {
            basis: self.basis,
            priority: self.priority,
        })
    }
}

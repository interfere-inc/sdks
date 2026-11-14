pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcern {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcernKind,
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    pub concern: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcernConcern,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcern {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcernBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcernBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcernBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcernKind>,
    target_id: Option<String>,
    concern: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcernConcern>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcernBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcernKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn target_id(mut self, value: impl Into<String>) -> Self {
        self.target_id = Some(value.into());
        self
    }

    pub fn concern(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcernConcern) -> Self {
        self.concern = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcern`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcernBuilder::kind)
    /// - [`concern`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcernBuilder::concern)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcern, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionConcern {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            target_id: self.target_id,
            concern: self.concern.ok_or_else(|| BuildError::missing_field("concern"))?,
        })
    }
}

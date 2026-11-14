pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemType {
    #[serde(default)]
    pub attrs: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemTypeAttrs,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemTypeType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemType {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemTypeBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemTypeBuilder {
    attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemTypeAttrs>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemTypeType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemType, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenCauseDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

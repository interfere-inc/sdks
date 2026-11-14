pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemType {
    #[serde(default)]
    pub attrs: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeAttrs,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemType {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeBuilder {
    attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeAttrs>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemType, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenOneDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemType {
    #[serde(default)]
    pub attrs: QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemTypeAttrs,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemTypeType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemType {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemTypeBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemTypeBuilder {
    attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemTypeAttrs>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemTypeType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemType, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentSixDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

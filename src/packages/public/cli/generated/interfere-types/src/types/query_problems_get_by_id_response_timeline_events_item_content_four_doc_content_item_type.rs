pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemType {
    #[serde(default)]
    pub attrs: QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemTypeAttrs,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemTypeType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemType {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemTypeBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemTypeBuilder {
    attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemTypeAttrs>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemTypeType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemType, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentFourDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

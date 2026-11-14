pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemType {
    #[serde(default)]
    pub attrs: QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemTypeAttrs,
    pub r#type: QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemTypeType,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemType {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemTypeBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemTypeBuilder {
    attrs: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemTypeAttrs>,
    r#type: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemTypeType>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemType, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentChartDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

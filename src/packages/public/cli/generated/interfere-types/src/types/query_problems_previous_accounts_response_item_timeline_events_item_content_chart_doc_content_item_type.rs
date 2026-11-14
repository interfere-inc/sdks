pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemType {
    #[serde(default)]
    pub attrs: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemTypeAttrs,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemTypeType,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemType {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemTypeBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemTypeBuilder {
    attrs: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemTypeAttrs>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemTypeType>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemType, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

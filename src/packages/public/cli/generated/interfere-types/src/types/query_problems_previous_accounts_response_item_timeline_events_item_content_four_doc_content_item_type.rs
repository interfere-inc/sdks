pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemType {
    #[serde(default)]
    pub attrs: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemTypeAttrs,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemTypeType,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemType {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemTypeBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemTypeBuilder {
    attrs: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemTypeAttrs>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemTypeType>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemType, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentFourDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

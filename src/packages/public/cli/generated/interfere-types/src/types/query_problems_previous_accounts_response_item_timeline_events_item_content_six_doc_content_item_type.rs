pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemType {
    #[serde(default)]
    pub attrs: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemTypeAttrs,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemTypeType,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemType {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemTypeBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemTypeBuilder {
    attrs: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemTypeAttrs>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemTypeType>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemType, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentSixDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

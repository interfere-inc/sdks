pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemType {
    #[serde(default)]
    pub attrs: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemTypeAttrs,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemTypeType,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemType {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemTypeBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemTypeBuilder {
    attrs: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemTypeAttrs>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemTypeType>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemType, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

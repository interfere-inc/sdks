pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemType {
    #[serde(default)]
    pub attrs: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemTypeAttrs,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemTypeType,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemType {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemTypeBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemTypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemTypeBuilder {
    attrs: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemTypeAttrs>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemTypeType>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemTypeBuilder {
    pub fn attrs(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemTypeAttrs) -> Self {
        self.attrs = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemTypeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemType`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attrs`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemTypeBuilder::attrs)
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemTypeBuilder::r#type)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemType, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionDocContentItemType {
            attrs: self.attrs.ok_or_else(|| BuildError::missing_field("attrs"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

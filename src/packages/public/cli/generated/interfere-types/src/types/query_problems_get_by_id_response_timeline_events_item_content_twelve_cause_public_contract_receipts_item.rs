pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCausePublicContractReceiptsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCausePublicContractReceiptsItem {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCausePublicContractReceiptsItemBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCausePublicContractReceiptsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCausePublicContractReceiptsItemBuilder {
    id: Option<String>,
    content_digest: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCausePublicContractReceiptsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCausePublicContractReceiptsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCausePublicContractReceiptsItemBuilder::id)
    /// - [`content_digest`](QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCausePublicContractReceiptsItemBuilder::content_digest)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCausePublicContractReceiptsItem, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCausePublicContractReceiptsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
        })
    }
}

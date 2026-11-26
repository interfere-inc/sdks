pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentTwelveCausePublicContractReceiptsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
}

impl TimelineEventContentTwelveCausePublicContractReceiptsItem {
    pub fn builder() -> TimelineEventContentTwelveCausePublicContractReceiptsItemBuilder {
        <TimelineEventContentTwelveCausePublicContractReceiptsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveCausePublicContractReceiptsItemBuilder {
    id: Option<String>,
    content_digest: Option<String>,
}

impl TimelineEventContentTwelveCausePublicContractReceiptsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveCausePublicContractReceiptsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TimelineEventContentTwelveCausePublicContractReceiptsItemBuilder::id)
    /// - [`content_digest`](TimelineEventContentTwelveCausePublicContractReceiptsItemBuilder::content_digest)
    pub fn build(self) -> Result<TimelineEventContentTwelveCausePublicContractReceiptsItem, BuildError> {
        Ok(TimelineEventContentTwelveCausePublicContractReceiptsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
        })
    }
}

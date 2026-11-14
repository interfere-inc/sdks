pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentFourDocContentItemTypeAttrs {
    #[serde(default)]
    pub alt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(default)]
    pub source: String,
}

impl TimelineEventContentFourDocContentItemTypeAttrs {
    pub fn builder() -> TimelineEventContentFourDocContentItemTypeAttrsBuilder {
        <TimelineEventContentFourDocContentItemTypeAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemTypeAttrsBuilder {
    alt: Option<String>,
    caption: Option<String>,
    source: Option<String>,
}

impl TimelineEventContentFourDocContentItemTypeAttrsBuilder {
    pub fn alt(mut self, value: impl Into<String>) -> Self {
        self.alt = Some(value.into());
        self
    }

    pub fn caption(mut self, value: impl Into<String>) -> Self {
        self.caption = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemTypeAttrs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`alt`](TimelineEventContentFourDocContentItemTypeAttrsBuilder::alt)
    /// - [`source`](TimelineEventContentFourDocContentItemTypeAttrsBuilder::source)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemTypeAttrs, BuildError> {
        Ok(TimelineEventContentFourDocContentItemTypeAttrs {
            alt: self.alt.ok_or_else(|| BuildError::missing_field("alt"))?,
            caption: self.caption,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}

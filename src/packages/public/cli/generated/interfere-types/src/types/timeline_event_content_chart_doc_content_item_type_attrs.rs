pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentChartDocContentItemTypeAttrs {
    #[serde(default)]
    pub alt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(default)]
    pub source: String,
}

impl TimelineEventContentChartDocContentItemTypeAttrs {
    pub fn builder() -> TimelineEventContentChartDocContentItemTypeAttrsBuilder {
        <TimelineEventContentChartDocContentItemTypeAttrsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartDocContentItemTypeAttrsBuilder {
    alt: Option<String>,
    caption: Option<String>,
    source: Option<String>,
}

impl TimelineEventContentChartDocContentItemTypeAttrsBuilder {
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

    /// Consumes the builder and constructs a [`TimelineEventContentChartDocContentItemTypeAttrs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`alt`](TimelineEventContentChartDocContentItemTypeAttrsBuilder::alt)
    /// - [`source`](TimelineEventContentChartDocContentItemTypeAttrsBuilder::source)
    pub fn build(self) -> Result<TimelineEventContentChartDocContentItemTypeAttrs, BuildError> {
        Ok(TimelineEventContentChartDocContentItemTypeAttrs {
            alt: self.alt.ok_or_else(|| BuildError::missing_field("alt"))?,
            caption: self.caption,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}

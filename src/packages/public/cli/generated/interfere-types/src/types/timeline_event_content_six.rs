pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentSix {
    pub doc: TimelineEventContentSixDoc,
    /// A short title for this block, around 30 characters. Name what the block tells the reader, not the investigation stage. The card truncates what does not fit, so a longer title costs the tail of the label and nothing else.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub version: String,
}

impl TimelineEventContentSix {
    pub fn builder() -> TimelineEventContentSixBuilder {
        <TimelineEventContentSixBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixBuilder {
    doc: Option<TimelineEventContentSixDoc>,
    title: Option<String>,
    version: Option<String>,
}

impl TimelineEventContentSixBuilder {
    pub fn doc(mut self, value: TimelineEventContentSixDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSix`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](TimelineEventContentSixBuilder::doc)
    /// - [`version`](TimelineEventContentSixBuilder::version)
    pub fn build(self) -> Result<TimelineEventContentSix, BuildError> {
        Ok(TimelineEventContentSix {
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
            title: self.title,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentElevenOne {
    pub doc: TimelineEventContentElevenOneDoc,
    #[serde(default)]
    pub version: String,
}

impl TimelineEventContentElevenOne {
    pub fn builder() -> TimelineEventContentElevenOneBuilder {
        <TimelineEventContentElevenOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenOneBuilder {
    doc: Option<TimelineEventContentElevenOneDoc>,
    version: Option<String>,
}

impl TimelineEventContentElevenOneBuilder {
    pub fn doc(mut self, value: TimelineEventContentElevenOneDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](TimelineEventContentElevenOneBuilder::doc)
    /// - [`version`](TimelineEventContentElevenOneBuilder::version)
    pub fn build(self) -> Result<TimelineEventContentElevenOne, BuildError> {
        Ok(TimelineEventContentElevenOne {
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}

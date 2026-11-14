pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentFour {
    pub doc: TimelineEventContentFourDoc,
    #[serde(default)]
    pub version: String,
}

impl TimelineEventContentFour {
    pub fn builder() -> TimelineEventContentFourBuilder {
        <TimelineEventContentFourBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourBuilder {
    doc: Option<TimelineEventContentFourDoc>,
    version: Option<String>,
}

impl TimelineEventContentFourBuilder {
    pub fn doc(mut self, value: TimelineEventContentFourDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFour`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](TimelineEventContentFourBuilder::doc)
    /// - [`version`](TimelineEventContentFourBuilder::version)
    pub fn build(self) -> Result<TimelineEventContentFour, BuildError> {
        Ok(TimelineEventContentFour {
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}

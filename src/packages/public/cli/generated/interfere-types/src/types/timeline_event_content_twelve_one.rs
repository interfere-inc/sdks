pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentTwelveOne {
    pub doc: TimelineEventContentTwelveOneDoc,
    #[serde(default)]
    pub version: String,
}

impl TimelineEventContentTwelveOne {
    pub fn builder() -> TimelineEventContentTwelveOneBuilder {
        <TimelineEventContentTwelveOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveOneBuilder {
    doc: Option<TimelineEventContentTwelveOneDoc>,
    version: Option<String>,
}

impl TimelineEventContentTwelveOneBuilder {
    pub fn doc(mut self, value: TimelineEventContentTwelveOneDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](TimelineEventContentTwelveOneBuilder::doc)
    /// - [`version`](TimelineEventContentTwelveOneBuilder::version)
    pub fn build(self) -> Result<TimelineEventContentTwelveOne, BuildError> {
        Ok(TimelineEventContentTwelveOne {
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentElevenCause {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<TimelineEventContentElevenCauseCause>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<TimelineEventContentElevenCauseConfidence>,
    pub doc: TimelineEventContentElevenCauseDoc,
    #[serde(default)]
    pub summary: String,
    /// A short title for this block, around 30 characters. Name what the block tells the reader, not the investigation stage. The card truncates what does not fit, so a longer title costs the tail of the label and nothing else.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unconfirmed: Option<String>,
    #[serde(default)]
    pub version: String,
}

impl TimelineEventContentElevenCause {
    pub fn builder() -> TimelineEventContentElevenCauseBuilder {
        <TimelineEventContentElevenCauseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentElevenCauseBuilder {
    cause: Option<TimelineEventContentElevenCauseCause>,
    confidence: Option<TimelineEventContentElevenCauseConfidence>,
    doc: Option<TimelineEventContentElevenCauseDoc>,
    summary: Option<String>,
    title: Option<String>,
    unconfirmed: Option<String>,
    version: Option<String>,
}

impl TimelineEventContentElevenCauseBuilder {
    pub fn cause(mut self, value: TimelineEventContentElevenCauseCause) -> Self {
        self.cause = Some(value);
        self
    }

    pub fn confidence(mut self, value: TimelineEventContentElevenCauseConfidence) -> Self {
        self.confidence = Some(value);
        self
    }

    pub fn doc(mut self, value: TimelineEventContentElevenCauseDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn unconfirmed(mut self, value: impl Into<String>) -> Self {
        self.unconfirmed = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentElevenCause`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](TimelineEventContentElevenCauseBuilder::doc)
    /// - [`summary`](TimelineEventContentElevenCauseBuilder::summary)
    /// - [`version`](TimelineEventContentElevenCauseBuilder::version)
    pub fn build(self) -> Result<TimelineEventContentElevenCause, BuildError> {
        Ok(TimelineEventContentElevenCause {
            cause: self.cause,
            confidence: self.confidence,
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
            summary: self.summary.ok_or_else(|| BuildError::missing_field("summary"))?,
            title: self.title,
            unconfirmed: self.unconfirmed,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}

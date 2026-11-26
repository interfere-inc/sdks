pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCause {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCause>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseConfidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrection>,
    pub doc: QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDoc,
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

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCause {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseBuilder {
    cause: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCause>,
    confidence: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseConfidence>,
    correction: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrection>,
    doc: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDoc>,
    summary: Option<String>,
    title: Option<String>,
    unconfirmed: Option<String>,
    version: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseBuilder {
    pub fn cause(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCause) -> Self {
        self.cause = Some(value);
        self
    }

    pub fn confidence(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseConfidence) -> Self {
        self.confidence = Some(value);
        self
    }

    pub fn correction(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseCorrection) -> Self {
        self.correction = Some(value);
        self
    }

    pub fn doc(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseDoc) -> Self {
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

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCause`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseBuilder::doc)
    /// - [`summary`](QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseBuilder::summary)
    /// - [`version`](QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCauseBuilder::version)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCause, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentTwelveCause {
            cause: self.cause,
            confidence: self.confidence,
            correction: self.correction,
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
            summary: self.summary.ok_or_else(|| BuildError::missing_field("summary"))?,
            title: self.title,
            unconfirmed: self.unconfirmed,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}

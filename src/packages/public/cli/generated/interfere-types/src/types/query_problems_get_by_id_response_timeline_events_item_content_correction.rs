pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentCorrection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrection>,
    pub doc: QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDoc,
    /// A short title for this block, around 30 characters. Name what the block tells the reader, not the investigation stage. The card truncates what does not fit, so a longer title costs the tail of the label and nothing else.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub version: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentCorrection {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionBuilder {
    correction: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrection>,
    doc: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDoc>,
    title: Option<String>,
    version: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionBuilder {
    pub fn correction(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrection) -> Self {
        self.correction = Some(value);
        self
    }

    pub fn doc(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionDoc) -> Self {
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

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentCorrection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionBuilder::doc)
    /// - [`version`](QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionBuilder::version)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrection, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentCorrection {
            correction: self.correction,
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
            title: self.title,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}

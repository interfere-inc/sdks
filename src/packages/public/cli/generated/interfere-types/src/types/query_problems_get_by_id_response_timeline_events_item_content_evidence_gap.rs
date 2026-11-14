pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGap {
    pub doc: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDoc,
    #[serde(rename = "evidenceGap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_gap: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGap>,
    #[serde(rename = "publicationLost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_lost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHold>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublication>,
    /// A short title for this block, around 30 characters. Name what the block tells the reader, not the investigation stage. The card truncates what does not fit, so a longer title costs the tail of the label and nothing else.
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub version: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGap {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapBuilder {
    doc: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDoc>,
    evidence_gap: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGap>,
    publication_lost: Option<String>,
    hold: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHold>,
    publication: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublication>,
    title: Option<String>,
    version: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapBuilder {
    pub fn doc(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn evidence_gap(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGap) -> Self {
        self.evidence_gap = Some(value);
        self
    }

    pub fn publication_lost(mut self, value: impl Into<String>) -> Self {
        self.publication_lost = Some(value.into());
        self
    }

    pub fn hold(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHold) -> Self {
        self.hold = Some(value);
        self
    }

    pub fn publication(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublication) -> Self {
        self.publication = Some(value);
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

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGap`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapBuilder::doc)
    /// - [`title`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapBuilder::title)
    /// - [`version`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapBuilder::version)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGap, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGap {
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
            evidence_gap: self.evidence_gap,
            publication_lost: self.publication_lost,
            hold: self.hold,
            publication: self.publication,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}

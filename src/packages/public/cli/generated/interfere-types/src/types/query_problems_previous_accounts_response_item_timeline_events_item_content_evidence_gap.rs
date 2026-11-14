pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGap {
    pub doc: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDoc,
    #[serde(rename = "evidenceGap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_gap: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGap>,
    #[serde(rename = "publicationLost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_lost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHold>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublication>,
    /// A short title for this block, around 30 characters. Name what the block tells the reader, not the investigation stage. The card truncates what does not fit, so a longer title costs the tail of the label and nothing else.
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub version: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGap {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapBuilder {
    doc: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDoc>,
    evidence_gap: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGap>,
    publication_lost: Option<String>,
    hold: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHold>,
    publication: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublication>,
    title: Option<String>,
    version: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapBuilder {
    pub fn doc(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn evidence_gap(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGap) -> Self {
        self.evidence_gap = Some(value);
        self
    }

    pub fn publication_lost(mut self, value: impl Into<String>) -> Self {
        self.publication_lost = Some(value.into());
        self
    }

    pub fn hold(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHold) -> Self {
        self.hold = Some(value);
        self
    }

    pub fn publication(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublication) -> Self {
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

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGap`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapBuilder::doc)
    /// - [`title`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapBuilder::title)
    /// - [`version`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapBuilder::version)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGap, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGap {
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

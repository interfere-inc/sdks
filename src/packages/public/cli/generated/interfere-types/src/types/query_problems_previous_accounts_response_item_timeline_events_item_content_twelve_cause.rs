pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCause {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCause>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseConfidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrection>,
    pub doc: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseDoc,
    #[serde(rename = "publicContractReceipts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_contract_receipts: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCausePublicContractReceiptsItem>>,
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

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCause {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseBuilder {
    cause: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCause>,
    confidence: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseConfidence>,
    correction: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrection>,
    doc: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseDoc>,
    public_contract_receipts: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCausePublicContractReceiptsItem>>,
    summary: Option<String>,
    title: Option<String>,
    unconfirmed: Option<String>,
    version: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseBuilder {
    pub fn cause(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCause) -> Self {
        self.cause = Some(value);
        self
    }

    pub fn confidence(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseConfidence) -> Self {
        self.confidence = Some(value);
        self
    }

    pub fn correction(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrection) -> Self {
        self.correction = Some(value);
        self
    }

    pub fn doc(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn public_contract_receipts(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCausePublicContractReceiptsItem>) -> Self {
        self.public_contract_receipts = Some(value);
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

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCause`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseBuilder::doc)
    /// - [`summary`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseBuilder::summary)
    /// - [`version`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseBuilder::version)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCause, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCause {
            cause: self.cause,
            confidence: self.confidence,
            correction: self.correction,
            doc: self.doc.ok_or_else(|| BuildError::missing_field("doc"))?,
            public_contract_receipts: self.public_contract_receipts,
            summary: self.summary.ok_or_else(|| BuildError::missing_field("summary"))?,
            title: self.title,
            unconfirmed: self.unconfirmed,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}

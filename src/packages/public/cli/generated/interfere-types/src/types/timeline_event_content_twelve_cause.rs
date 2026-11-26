pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentTwelveCause {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<TimelineEventContentTwelveCauseCause>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<TimelineEventContentTwelveCauseConfidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction: Option<TimelineEventContentTwelveCauseCorrection>,
    pub doc: TimelineEventContentTwelveCauseDoc,
    #[serde(rename = "publicContractReceipts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_contract_receipts: Option<Vec<TimelineEventContentTwelveCausePublicContractReceiptsItem>>,
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

impl TimelineEventContentTwelveCause {
    pub fn builder() -> TimelineEventContentTwelveCauseBuilder {
        <TimelineEventContentTwelveCauseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTwelveCauseBuilder {
    cause: Option<TimelineEventContentTwelveCauseCause>,
    confidence: Option<TimelineEventContentTwelveCauseConfidence>,
    correction: Option<TimelineEventContentTwelveCauseCorrection>,
    doc: Option<TimelineEventContentTwelveCauseDoc>,
    public_contract_receipts: Option<Vec<TimelineEventContentTwelveCausePublicContractReceiptsItem>>,
    summary: Option<String>,
    title: Option<String>,
    unconfirmed: Option<String>,
    version: Option<String>,
}

impl TimelineEventContentTwelveCauseBuilder {
    pub fn cause(mut self, value: TimelineEventContentTwelveCauseCause) -> Self {
        self.cause = Some(value);
        self
    }

    pub fn confidence(mut self, value: TimelineEventContentTwelveCauseConfidence) -> Self {
        self.confidence = Some(value);
        self
    }

    pub fn correction(mut self, value: TimelineEventContentTwelveCauseCorrection) -> Self {
        self.correction = Some(value);
        self
    }

    pub fn doc(mut self, value: TimelineEventContentTwelveCauseDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn public_contract_receipts(mut self, value: Vec<TimelineEventContentTwelveCausePublicContractReceiptsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`TimelineEventContentTwelveCause`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doc`](TimelineEventContentTwelveCauseBuilder::doc)
    /// - [`summary`](TimelineEventContentTwelveCauseBuilder::summary)
    /// - [`version`](TimelineEventContentTwelveCauseBuilder::version)
    pub fn build(self) -> Result<TimelineEventContentTwelveCause, BuildError> {
        Ok(TimelineEventContentTwelveCause {
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

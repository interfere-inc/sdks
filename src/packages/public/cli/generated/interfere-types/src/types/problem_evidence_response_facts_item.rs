pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseFactsItem {
    #[serde(rename = "factId")]
    #[serde(default)]
    pub fact_id: String,
    #[serde(default)]
    pub timestamp: String,
    #[serde(rename = "errorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl ProblemEvidenceResponseFactsItem {
    pub fn builder() -> ProblemEvidenceResponseFactsItemBuilder {
        <ProblemEvidenceResponseFactsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseFactsItemBuilder {
    fact_id: Option<String>,
    timestamp: Option<String>,
    error_name: Option<String>,
    error_message: Option<String>,
    session_id: Option<String>,
}

impl ProblemEvidenceResponseFactsItemBuilder {
    pub fn fact_id(mut self, value: impl Into<String>) -> Self {
        self.fact_id = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: impl Into<String>) -> Self {
        self.timestamp = Some(value.into());
        self
    }

    pub fn error_name(mut self, value: impl Into<String>) -> Self {
        self.error_name = Some(value.into());
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseFactsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fact_id`](ProblemEvidenceResponseFactsItemBuilder::fact_id)
    /// - [`timestamp`](ProblemEvidenceResponseFactsItemBuilder::timestamp)
    pub fn build(self) -> Result<ProblemEvidenceResponseFactsItem, BuildError> {
        Ok(ProblemEvidenceResponseFactsItem {
            fact_id: self.fact_id.ok_or_else(|| BuildError::missing_field("fact_id"))?,
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
            error_name: self.error_name,
            error_message: self.error_message,
            session_id: self.session_id,
        })
    }
}

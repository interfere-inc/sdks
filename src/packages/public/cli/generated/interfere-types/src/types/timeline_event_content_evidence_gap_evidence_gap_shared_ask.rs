pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentEvidenceGapEvidenceGapSharedAsk {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(default)]
    pub sequence: i64,
}

impl TimelineEventContentEvidenceGapEvidenceGapSharedAsk {
    pub fn builder() -> TimelineEventContentEvidenceGapEvidenceGapSharedAskBuilder {
        <TimelineEventContentEvidenceGapEvidenceGapSharedAskBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapEvidenceGapSharedAskBuilder {
    problem_id: Option<String>,
    sequence: Option<i64>,
}

impl TimelineEventContentEvidenceGapEvidenceGapSharedAskBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapEvidenceGapSharedAsk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](TimelineEventContentEvidenceGapEvidenceGapSharedAskBuilder::problem_id)
    /// - [`sequence`](TimelineEventContentEvidenceGapEvidenceGapSharedAskBuilder::sequence)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapEvidenceGapSharedAsk, BuildError> {
        Ok(TimelineEventContentEvidenceGapEvidenceGapSharedAsk {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            sequence: self.sequence.ok_or_else(|| BuildError::missing_field("sequence"))?,
        })
    }
}

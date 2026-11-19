pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentDismissalDismissalProblemId {
    pub kind: TimelineEventContentDismissalDismissalProblemIdKind,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl TimelineEventContentDismissalDismissalProblemId {
    pub fn builder() -> TimelineEventContentDismissalDismissalProblemIdBuilder {
        <TimelineEventContentDismissalDismissalProblemIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentDismissalDismissalProblemIdBuilder {
    kind: Option<TimelineEventContentDismissalDismissalProblemIdKind>,
    problem_id: Option<String>,
}

impl TimelineEventContentDismissalDismissalProblemIdBuilder {
    pub fn kind(mut self, value: TimelineEventContentDismissalDismissalProblemIdKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentDismissalDismissalProblemId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventContentDismissalDismissalProblemIdBuilder::kind)
    /// - [`problem_id`](TimelineEventContentDismissalDismissalProblemIdBuilder::problem_id)
    pub fn build(self) -> Result<TimelineEventContentDismissalDismissalProblemId, BuildError> {
        Ok(TimelineEventContentDismissalDismissalProblemId {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}

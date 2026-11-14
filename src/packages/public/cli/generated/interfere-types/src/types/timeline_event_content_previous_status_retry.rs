pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentPreviousStatusRetry {
    pub kind: TimelineEventContentPreviousStatusRetryKind,
}

impl TimelineEventContentPreviousStatusRetry {
    pub fn builder() -> TimelineEventContentPreviousStatusRetryBuilder {
        <TimelineEventContentPreviousStatusRetryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentPreviousStatusRetryBuilder {
    kind: Option<TimelineEventContentPreviousStatusRetryKind>,
}

impl TimelineEventContentPreviousStatusRetryBuilder {
    pub fn kind(mut self, value: TimelineEventContentPreviousStatusRetryKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentPreviousStatusRetry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventContentPreviousStatusRetryBuilder::kind)
    pub fn build(self) -> Result<TimelineEventContentPreviousStatusRetry, BuildError> {
        Ok(TimelineEventContentPreviousStatusRetry {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}

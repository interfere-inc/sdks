pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentDismissalRetry {
    pub kind: TimelineEventContentDismissalRetryKind,
}

impl TimelineEventContentDismissalRetry {
    pub fn builder() -> TimelineEventContentDismissalRetryBuilder {
        <TimelineEventContentDismissalRetryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentDismissalRetryBuilder {
    kind: Option<TimelineEventContentDismissalRetryKind>,
}

impl TimelineEventContentDismissalRetryBuilder {
    pub fn kind(mut self, value: TimelineEventContentDismissalRetryKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentDismissalRetry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventContentDismissalRetryBuilder::kind)
    pub fn build(self) -> Result<TimelineEventContentDismissalRetry, BuildError> {
        Ok(TimelineEventContentDismissalRetry {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}

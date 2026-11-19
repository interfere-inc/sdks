pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentDismissalDismissalZero {
    pub kind: TimelineEventContentDismissalDismissalZeroKind,
}

impl TimelineEventContentDismissalDismissalZero {
    pub fn builder() -> TimelineEventContentDismissalDismissalZeroBuilder {
        <TimelineEventContentDismissalDismissalZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentDismissalDismissalZeroBuilder {
    kind: Option<TimelineEventContentDismissalDismissalZeroKind>,
}

impl TimelineEventContentDismissalDismissalZeroBuilder {
    pub fn kind(mut self, value: TimelineEventContentDismissalDismissalZeroKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentDismissalDismissalZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventContentDismissalDismissalZeroBuilder::kind)
    pub fn build(self) -> Result<TimelineEventContentDismissalDismissalZero, BuildError> {
        Ok(TimelineEventContentDismissalDismissalZero {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}

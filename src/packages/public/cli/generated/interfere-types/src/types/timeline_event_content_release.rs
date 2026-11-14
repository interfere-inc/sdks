pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct TimelineEventContentRelease {
    pub release: Option<TimelineEventContentReleaseRelease>,
}

impl TimelineEventContentRelease {
    pub fn builder() -> TimelineEventContentReleaseBuilder {
        <TimelineEventContentReleaseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentReleaseBuilder {
    release: Option<TimelineEventContentReleaseRelease>,
}

impl TimelineEventContentReleaseBuilder {
    pub fn release(mut self, value: TimelineEventContentReleaseRelease) -> Self {
        self.release = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentRelease`].
    pub fn build(self) -> Result<TimelineEventContentRelease, BuildError> {
        Ok(TimelineEventContentRelease {
            release: self.release,
        })
    }
}

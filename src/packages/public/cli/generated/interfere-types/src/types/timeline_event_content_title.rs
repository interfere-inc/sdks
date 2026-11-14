pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct TimelineEventContentTitle {
    pub title: String,
}

impl TimelineEventContentTitle {
    pub fn builder() -> TimelineEventContentTitleBuilder {
        <TimelineEventContentTitleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTitleBuilder {
    title: Option<String>,
}

impl TimelineEventContentTitleBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTitle`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](TimelineEventContentTitleBuilder::title)
    pub fn build(self) -> Result<TimelineEventContentTitle, BuildError> {
        Ok(TimelineEventContentTitle {
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}

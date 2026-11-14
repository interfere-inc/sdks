pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct TimelineEventContentDescription {
    pub description: String,
}

impl TimelineEventContentDescription {
    pub fn builder() -> TimelineEventContentDescriptionBuilder {
        <TimelineEventContentDescriptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentDescriptionBuilder {
    description: Option<String>,
}

impl TimelineEventContentDescriptionBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentDescription`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](TimelineEventContentDescriptionBuilder::description)
    pub fn build(self) -> Result<TimelineEventContentDescription, BuildError> {
        Ok(TimelineEventContentDescription {
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}

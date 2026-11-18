pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentActionKindService {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl TimelineEventContentActionKindService {
    pub fn builder() -> TimelineEventContentActionKindServiceBuilder {
        <TimelineEventContentActionKindServiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindServiceBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl TimelineEventContentActionKindServiceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentActionKindService`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TimelineEventContentActionKindServiceBuilder::id)
    /// - [`name`](TimelineEventContentActionKindServiceBuilder::name)
    pub fn build(self) -> Result<TimelineEventContentActionKindService, BuildError> {
        Ok(TimelineEventContentActionKindService {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

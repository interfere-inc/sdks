pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentActionKindOptionsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
}

impl TimelineEventContentActionKindOptionsItem {
    pub fn builder() -> TimelineEventContentActionKindOptionsItemBuilder {
        <TimelineEventContentActionKindOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindOptionsItemBuilder {
    id: Option<String>,
    label: Option<String>,
}

impl TimelineEventContentActionKindOptionsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentActionKindOptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TimelineEventContentActionKindOptionsItemBuilder::id)
    /// - [`label`](TimelineEventContentActionKindOptionsItemBuilder::label)
    pub fn build(self) -> Result<TimelineEventContentActionKindOptionsItem, BuildError> {
        Ok(TimelineEventContentActionKindOptionsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}

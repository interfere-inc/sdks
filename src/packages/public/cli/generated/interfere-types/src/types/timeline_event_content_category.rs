pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct TimelineEventContentCategory {
    pub category: Option<TimelineEventContentCategoryCategory>,
}

impl TimelineEventContentCategory {
    pub fn builder() -> TimelineEventContentCategoryBuilder {
        <TimelineEventContentCategoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCategoryBuilder {
    category: Option<TimelineEventContentCategoryCategory>,
}

impl TimelineEventContentCategoryBuilder {
    pub fn category(mut self, value: TimelineEventContentCategoryCategory) -> Self {
        self.category = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCategory`].
    pub fn build(self) -> Result<TimelineEventContentCategory, BuildError> {
        Ok(TimelineEventContentCategory {
            category: self.category,
        })
    }
}

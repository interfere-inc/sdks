pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindOptionsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindOptionsItem {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindOptionsItemBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindOptionsItemBuilder {
    id: Option<String>,
    label: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindOptionsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindOptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindOptionsItemBuilder::id)
    /// - [`label`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindOptionsItemBuilder::label)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindOptionsItem, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindOptionsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}

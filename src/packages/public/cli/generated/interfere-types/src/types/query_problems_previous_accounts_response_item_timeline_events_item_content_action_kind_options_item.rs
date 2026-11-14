pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindOptionsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindOptionsItem {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindOptionsItemBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindOptionsItemBuilder {
    id: Option<String>,
    label: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindOptionsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindOptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindOptionsItemBuilder::id)
    /// - [`label`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindOptionsItemBuilder::label)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindOptionsItem, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindOptionsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}

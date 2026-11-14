pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription {
    pub description: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescriptionBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescriptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescriptionBuilder {
    description: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescriptionBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescriptionBuilder::description)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDescription {
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}

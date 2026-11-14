pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentDescription {
    pub description: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentDescription {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentDescriptionBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentDescriptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentDescriptionBuilder {
    description: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentDescriptionBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentDescription`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](QueryProblemsGetByIdResponseTimelineEventsItemContentDescriptionBuilder::description)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentDescription, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentDescription {
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}

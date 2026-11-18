pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindService {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindService {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindServiceBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindServiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindServiceBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindServiceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindService`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindServiceBuilder::id)
    /// - [`name`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindServiceBuilder::name)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindService, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindService {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepository {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub provider: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepository {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepositoryBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepositoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepositoryBuilder {
    id: Option<String>,
    name: Option<String>,
    url: Option<String>,
    provider: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepositoryBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepository`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepositoryBuilder::id)
    /// - [`name`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepositoryBuilder::name)
    /// - [`url`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepositoryBuilder::url)
    /// - [`provider`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepositoryBuilder::provider)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepository, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRepository {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}

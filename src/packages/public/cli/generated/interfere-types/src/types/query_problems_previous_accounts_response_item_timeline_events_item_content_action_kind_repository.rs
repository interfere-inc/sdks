pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRepository {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub provider: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRepository {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRepositoryBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRepositoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRepositoryBuilder {
    id: Option<String>,
    name: Option<String>,
    url: Option<String>,
    provider: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRepositoryBuilder {
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

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRepository`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRepositoryBuilder::id)
    /// - [`name`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRepositoryBuilder::name)
    /// - [`url`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRepositoryBuilder::url)
    /// - [`provider`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRepositoryBuilder::provider)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRepository, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindRepository {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}

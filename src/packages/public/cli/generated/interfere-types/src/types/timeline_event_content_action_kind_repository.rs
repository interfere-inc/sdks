pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentActionKindRepository {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub provider: String,
}

impl TimelineEventContentActionKindRepository {
    pub fn builder() -> TimelineEventContentActionKindRepositoryBuilder {
        <TimelineEventContentActionKindRepositoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindRepositoryBuilder {
    id: Option<String>,
    name: Option<String>,
    url: Option<String>,
    provider: Option<String>,
}

impl TimelineEventContentActionKindRepositoryBuilder {
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

    /// Consumes the builder and constructs a [`TimelineEventContentActionKindRepository`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TimelineEventContentActionKindRepositoryBuilder::id)
    /// - [`name`](TimelineEventContentActionKindRepositoryBuilder::name)
    /// - [`url`](TimelineEventContentActionKindRepositoryBuilder::url)
    /// - [`provider`](TimelineEventContentActionKindRepositoryBuilder::provider)
    pub fn build(self) -> Result<TimelineEventContentActionKindRepository, BuildError> {
        Ok(TimelineEventContentActionKindRepository {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}

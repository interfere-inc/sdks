pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for count
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CountQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(rename = "searchActions")]
    #[serde(default)]
    pub search_actions: Vec<Option<String>>,
    #[serde(default)]
    pub actions: Vec<Option<String>>,
    #[serde(default)]
    pub actors: Vec<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub since: Option<f64>,
}

impl CountQueryRequest {
    pub fn builder() -> CountQueryRequestBuilder {
        <CountQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CountQueryRequestBuilder {
    search: Option<String>,
    search_actions: Option<Vec<Option<String>>>,
    actions: Option<Vec<Option<String>>>,
    actors: Option<Vec<Option<String>>>,
    since: Option<f64>,
}

impl CountQueryRequestBuilder {
    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn search_actions(mut self, value: Vec<Option<String>>) -> Self {
        self.search_actions = Some(value);
        self
    }

    pub fn actions(mut self, value: Vec<Option<String>>) -> Self {
        self.actions = Some(value);
        self
    }

    pub fn actors(mut self, value: Vec<Option<String>>) -> Self {
        self.actors = Some(value);
        self
    }

    pub fn since(mut self, value: f64) -> Self {
        self.since = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CountQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`search_actions`](CountQueryRequestBuilder::search_actions)
    /// - [`actions`](CountQueryRequestBuilder::actions)
    /// - [`actors`](CountQueryRequestBuilder::actors)
    pub fn build(self) -> Result<CountQueryRequest, BuildError> {
        Ok(CountQueryRequest {
            search: self.search,
            search_actions: self.search_actions.ok_or_else(|| BuildError::missing_field("search_actions"))?,
            actions: self.actions.ok_or_else(|| BuildError::missing_field("actions"))?,
            actors: self.actors.ok_or_else(|| BuildError::missing_field("actors"))?,
            since: self.since,
        })
    }
}


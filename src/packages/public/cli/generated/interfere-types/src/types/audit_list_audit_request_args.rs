pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAuditRequestArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(rename = "searchActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_actions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub since: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<ListAuditRequestArgsStart>,
}

impl ListAuditRequestArgs {
    pub fn builder() -> ListAuditRequestArgsBuilder {
        <ListAuditRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAuditRequestArgsBuilder {
    search: Option<String>,
    search_actions: Option<Vec<String>>,
    actions: Option<Vec<String>>,
    actors: Option<Vec<String>>,
    since: Option<f64>,
    limit: Option<i64>,
    start: Option<ListAuditRequestArgsStart>,
}

impl ListAuditRequestArgsBuilder {
    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn search_actions(mut self, value: Vec<String>) -> Self {
        self.search_actions = Some(value);
        self
    }

    pub fn actions(mut self, value: Vec<String>) -> Self {
        self.actions = Some(value);
        self
    }

    pub fn actors(mut self, value: Vec<String>) -> Self {
        self.actors = Some(value);
        self
    }

    pub fn since(mut self, value: f64) -> Self {
        self.since = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn start(mut self, value: ListAuditRequestArgsStart) -> Self {
        self.start = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAuditRequestArgs`].
    pub fn build(self) -> Result<ListAuditRequestArgs, BuildError> {
        Ok(ListAuditRequestArgs {
            search: self.search,
            search_actions: self.search_actions,
            actions: self.actions,
            actors: self.actors,
            since: self.since,
            limit: self.limit,
            start: self.start,
        })
    }
}

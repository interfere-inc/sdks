pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PageTenantsRequestArgs {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub limit: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "lastSeenAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_after: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<PageTenantsRequestArgsSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<PageTenantsRequestArgsDirection>,
}

impl PageTenantsRequestArgs {
    pub fn builder() -> PageTenantsRequestArgsBuilder {
        <PageTenantsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageTenantsRequestArgsBuilder {
    limit: Option<f64>,
    search: Option<String>,
    last_seen_after: Option<i64>,
    tiers: Option<Vec<String>>,
    sort: Option<PageTenantsRequestArgsSort>,
    direction: Option<PageTenantsRequestArgsDirection>,
}

impl PageTenantsRequestArgsBuilder {
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn last_seen_after(mut self, value: i64) -> Self {
        self.last_seen_after = Some(value);
        self
    }

    pub fn tiers(mut self, value: Vec<String>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn sort(mut self, value: PageTenantsRequestArgsSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn direction(mut self, value: PageTenantsRequestArgsDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PageTenantsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`limit`](PageTenantsRequestArgsBuilder::limit)
    pub fn build(self) -> Result<PageTenantsRequestArgs, BuildError> {
        Ok(PageTenantsRequestArgs {
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            search: self.search,
            last_seen_after: self.last_seen_after,
            tiers: self.tiers,
            sort: self.sort,
            direction: self.direction,
        })
    }
}

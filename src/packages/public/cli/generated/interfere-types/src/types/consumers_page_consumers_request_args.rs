pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PageConsumersRequestArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<String>>,
    #[serde(rename = "endUserOrgIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_user_org_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<PageConsumersRequestArgsIdentity>,
    #[serde(rename = "lastSeenAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_after: Option<i64>,
    #[serde(rename = "withoutCompany")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_company: Option<bool>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub limit: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<PageConsumersRequestArgsStart>,
    pub dir: PageConsumersRequestArgsDir,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

impl PageConsumersRequestArgs {
    pub fn builder() -> PageConsumersRequestArgsBuilder {
        <PageConsumersRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageConsumersRequestArgsBuilder {
    countries: Option<Vec<String>>,
    end_user_org_ids: Option<Vec<String>>,
    identity: Option<PageConsumersRequestArgsIdentity>,
    last_seen_after: Option<i64>,
    without_company: Option<bool>,
    limit: Option<f64>,
    start: Option<PageConsumersRequestArgsStart>,
    dir: Option<PageConsumersRequestArgsDir>,
    search: Option<String>,
}

impl PageConsumersRequestArgsBuilder {
    pub fn countries(mut self, value: Vec<String>) -> Self {
        self.countries = Some(value);
        self
    }

    pub fn end_user_org_ids(mut self, value: Vec<String>) -> Self {
        self.end_user_org_ids = Some(value);
        self
    }

    pub fn identity(mut self, value: PageConsumersRequestArgsIdentity) -> Self {
        self.identity = Some(value);
        self
    }

    pub fn last_seen_after(mut self, value: i64) -> Self {
        self.last_seen_after = Some(value);
        self
    }

    pub fn without_company(mut self, value: bool) -> Self {
        self.without_company = Some(value);
        self
    }

    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn start(mut self, value: PageConsumersRequestArgsStart) -> Self {
        self.start = Some(value);
        self
    }

    pub fn dir(mut self, value: PageConsumersRequestArgsDir) -> Self {
        self.dir = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PageConsumersRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`limit`](PageConsumersRequestArgsBuilder::limit)
    /// - [`dir`](PageConsumersRequestArgsBuilder::dir)
    pub fn build(self) -> Result<PageConsumersRequestArgs, BuildError> {
        Ok(PageConsumersRequestArgs {
            countries: self.countries,
            end_user_org_ids: self.end_user_org_ids,
            identity: self.identity,
            last_seen_after: self.last_seen_after,
            without_company: self.without_company,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            start: self.start,
            dir: self.dir.ok_or_else(|| BuildError::missing_field("dir"))?,
            search: self.search,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for findingOccurrences
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FindingOccurrencesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "releaseSlug")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_slug: Option<String>,
}

impl FindingOccurrencesQueryRequest {
    pub fn builder() -> FindingOccurrencesQueryRequestBuilder {
        <FindingOccurrencesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FindingOccurrencesQueryRequestBuilder {
    cursor: Option<String>,
    environment: Option<String>,
    limit: Option<i64>,
    release_slug: Option<String>,
}

impl FindingOccurrencesQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn release_slug(mut self, value: impl Into<String>) -> Self {
        self.release_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FindingOccurrencesQueryRequest`].
    pub fn build(self) -> Result<FindingOccurrencesQueryRequest, BuildError> {
        Ok(FindingOccurrencesQueryRequest {
            cursor: self.cursor,
            environment: self.environment,
            limit: self.limit,
            release_slug: self.release_slug,
        })
    }
}


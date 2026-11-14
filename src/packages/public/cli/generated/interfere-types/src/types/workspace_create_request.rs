pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkspaceCreateRequest {
    #[serde(rename = "dataResidencyLocation")]
    pub data_residency_location: WorkspaceCreateRequestDataResidencyLocation,
    #[serde(rename = "residencyPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residency_policy: Option<WorkspaceCreateRequestResidencyPolicy>,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub name: String,
    /// Workspace URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(default)]
    pub slug: String,
}

impl WorkspaceCreateRequest {
    pub fn builder() -> WorkspaceCreateRequestBuilder {
        <WorkspaceCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceCreateRequestBuilder {
    data_residency_location: Option<WorkspaceCreateRequestDataResidencyLocation>,
    residency_policy: Option<WorkspaceCreateRequestResidencyPolicy>,
    domain: Option<String>,
    name: Option<String>,
    slug: Option<String>,
}

impl WorkspaceCreateRequestBuilder {
    pub fn data_residency_location(mut self, value: WorkspaceCreateRequestDataResidencyLocation) -> Self {
        self.data_residency_location = Some(value);
        self
    }

    pub fn residency_policy(mut self, value: WorkspaceCreateRequestResidencyPolicy) -> Self {
        self.residency_policy = Some(value);
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data_residency_location`](WorkspaceCreateRequestBuilder::data_residency_location)
    /// - [`domain`](WorkspaceCreateRequestBuilder::domain)
    /// - [`name`](WorkspaceCreateRequestBuilder::name)
    /// - [`slug`](WorkspaceCreateRequestBuilder::slug)
    pub fn build(self) -> Result<WorkspaceCreateRequest, BuildError> {
        Ok(WorkspaceCreateRequest {
            data_residency_location: self.data_residency_location.ok_or_else(|| BuildError::missing_field("data_residency_location"))?,
            residency_policy: self.residency_policy,
            domain: self.domain.ok_or_else(|| BuildError::missing_field("domain"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}


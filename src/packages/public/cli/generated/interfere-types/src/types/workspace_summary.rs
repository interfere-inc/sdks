pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceSummary {
    #[serde(rename = "clerkOrgId")]
    #[serde(default)]
    pub clerk_org_id: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "imageUrl")]
    #[serde(default)]
    pub image_url: String,
    #[serde(default)]
    pub name: String,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "onboardingCompletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboarding_completed_at: Option<i64>,
    #[serde(default)]
    pub slug: String,
}

impl WorkspaceSummary {
    pub fn builder() -> WorkspaceSummaryBuilder {
        <WorkspaceSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceSummaryBuilder {
    clerk_org_id: Option<String>,
    id: Option<String>,
    image_url: Option<String>,
    name: Option<String>,
    onboarding_completed_at: Option<i64>,
    slug: Option<String>,
}

impl WorkspaceSummaryBuilder {
    pub fn clerk_org_id(mut self, value: impl Into<String>) -> Self {
        self.clerk_org_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn onboarding_completed_at(mut self, value: i64) -> Self {
        self.onboarding_completed_at = Some(value);
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`clerk_org_id`](WorkspaceSummaryBuilder::clerk_org_id)
    /// - [`id`](WorkspaceSummaryBuilder::id)
    /// - [`image_url`](WorkspaceSummaryBuilder::image_url)
    /// - [`name`](WorkspaceSummaryBuilder::name)
    /// - [`slug`](WorkspaceSummaryBuilder::slug)
    pub fn build(self) -> Result<WorkspaceSummary, BuildError> {
        Ok(WorkspaceSummary {
            clerk_org_id: self.clerk_org_id.ok_or_else(|| BuildError::missing_field("clerk_org_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            image_url: self.image_url.ok_or_else(|| BuildError::missing_field("image_url"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            onboarding_completed_at: self.onboarding_completed_at,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryMembershipsMineResponseItemOrganization {
    #[serde(rename = "allowMemberWorkspaceCreation")]
    #[serde(default)]
    pub allow_member_workspace_creation: bool,
    #[serde(rename = "autoAssignProblems")]
    #[serde(default)]
    pub auto_assign_problems: bool,
    #[serde(rename = "autoCreateApps")]
    #[serde(default)]
    pub auto_create_apps: bool,
    #[serde(rename = "clerkOrgId")]
    #[serde(default)]
    pub clerk_org_id: String,
    #[serde(rename = "clerkUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub clerk_updated_at: Option<f64>,
    #[serde(rename = "dataResidencyLocation")]
    pub data_residency_location: QueryMembershipsMineResponseItemOrganizationDataResidencyLocation,
    #[serde(rename = "defaultSurfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_surface_id: Option<String>,
    #[serde(rename = "demoResetAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub demo_reset_at: Option<f64>,
    #[serde(rename = "demoResetStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demo_reset_status: Option<QueryMembershipsMineResponseItemOrganizationDemoResetStatus>,
    #[serde(rename = "demoWorkspace")]
    #[serde(default)]
    pub demo_workspace: bool,
    #[serde(rename = "enabledAuthMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_auth_methods: Option<Vec<QueryMembershipsMineResponseItemOrganizationEnabledAuthMethodsItem>>,
    #[serde(rename = "environmentFiltering")]
    #[serde(default)]
    pub environment_filtering: bool,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "imageUrl")]
    #[serde(default)]
    pub image_url: String,
    #[serde(rename = "impersonationExpiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub impersonation_expires_at: Option<f64>,
    #[serde(rename = "impersonationGranted")]
    #[serde(default)]
    pub impersonation_granted: bool,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "onboardingCompletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub onboarding_completed_at: Option<f64>,
    #[serde(rename = "residencyPolicy")]
    pub residency_policy: QueryMembershipsMineResponseItemOrganizationResidencyPolicy,
    #[serde(default)]
    pub slug: String,
    #[serde(rename = "stripeCustomerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_customer_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub deleted_at: Option<f64>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub updated_at: f64,
}

impl QueryMembershipsMineResponseItemOrganization {
    pub fn builder() -> QueryMembershipsMineResponseItemOrganizationBuilder {
        <QueryMembershipsMineResponseItemOrganizationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryMembershipsMineResponseItemOrganizationBuilder {
    allow_member_workspace_creation: Option<bool>,
    auto_assign_problems: Option<bool>,
    auto_create_apps: Option<bool>,
    clerk_org_id: Option<String>,
    clerk_updated_at: Option<f64>,
    data_residency_location: Option<QueryMembershipsMineResponseItemOrganizationDataResidencyLocation>,
    default_surface_id: Option<String>,
    demo_reset_at: Option<f64>,
    demo_reset_status: Option<QueryMembershipsMineResponseItemOrganizationDemoResetStatus>,
    demo_workspace: Option<bool>,
    enabled_auth_methods: Option<Vec<QueryMembershipsMineResponseItemOrganizationEnabledAuthMethodsItem>>,
    environment_filtering: Option<bool>,
    id: Option<String>,
    image_url: Option<String>,
    impersonation_expires_at: Option<f64>,
    impersonation_granted: Option<bool>,
    name: Option<String>,
    onboarding_completed_at: Option<f64>,
    residency_policy: Option<QueryMembershipsMineResponseItemOrganizationResidencyPolicy>,
    slug: Option<String>,
    stripe_customer_id: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QueryMembershipsMineResponseItemOrganizationBuilder {
    pub fn allow_member_workspace_creation(mut self, value: bool) -> Self {
        self.allow_member_workspace_creation = Some(value);
        self
    }

    pub fn auto_assign_problems(mut self, value: bool) -> Self {
        self.auto_assign_problems = Some(value);
        self
    }

    pub fn auto_create_apps(mut self, value: bool) -> Self {
        self.auto_create_apps = Some(value);
        self
    }

    pub fn clerk_org_id(mut self, value: impl Into<String>) -> Self {
        self.clerk_org_id = Some(value.into());
        self
    }

    pub fn clerk_updated_at(mut self, value: f64) -> Self {
        self.clerk_updated_at = Some(value);
        self
    }

    pub fn data_residency_location(mut self, value: QueryMembershipsMineResponseItemOrganizationDataResidencyLocation) -> Self {
        self.data_residency_location = Some(value);
        self
    }

    pub fn default_surface_id(mut self, value: impl Into<String>) -> Self {
        self.default_surface_id = Some(value.into());
        self
    }

    pub fn demo_reset_at(mut self, value: f64) -> Self {
        self.demo_reset_at = Some(value);
        self
    }

    pub fn demo_reset_status(mut self, value: QueryMembershipsMineResponseItemOrganizationDemoResetStatus) -> Self {
        self.demo_reset_status = Some(value);
        self
    }

    pub fn demo_workspace(mut self, value: bool) -> Self {
        self.demo_workspace = Some(value);
        self
    }

    pub fn enabled_auth_methods(mut self, value: Vec<QueryMembershipsMineResponseItemOrganizationEnabledAuthMethodsItem>) -> Self {
        self.enabled_auth_methods = Some(value);
        self
    }

    pub fn environment_filtering(mut self, value: bool) -> Self {
        self.environment_filtering = Some(value);
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

    pub fn impersonation_expires_at(mut self, value: f64) -> Self {
        self.impersonation_expires_at = Some(value);
        self
    }

    pub fn impersonation_granted(mut self, value: bool) -> Self {
        self.impersonation_granted = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn onboarding_completed_at(mut self, value: f64) -> Self {
        self.onboarding_completed_at = Some(value);
        self
    }

    pub fn residency_policy(mut self, value: QueryMembershipsMineResponseItemOrganizationResidencyPolicy) -> Self {
        self.residency_policy = Some(value);
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn stripe_customer_id(mut self, value: impl Into<String>) -> Self {
        self.stripe_customer_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: f64) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: f64) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryMembershipsMineResponseItemOrganization`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allow_member_workspace_creation`](QueryMembershipsMineResponseItemOrganizationBuilder::allow_member_workspace_creation)
    /// - [`auto_assign_problems`](QueryMembershipsMineResponseItemOrganizationBuilder::auto_assign_problems)
    /// - [`auto_create_apps`](QueryMembershipsMineResponseItemOrganizationBuilder::auto_create_apps)
    /// - [`clerk_org_id`](QueryMembershipsMineResponseItemOrganizationBuilder::clerk_org_id)
    /// - [`data_residency_location`](QueryMembershipsMineResponseItemOrganizationBuilder::data_residency_location)
    /// - [`demo_workspace`](QueryMembershipsMineResponseItemOrganizationBuilder::demo_workspace)
    /// - [`environment_filtering`](QueryMembershipsMineResponseItemOrganizationBuilder::environment_filtering)
    /// - [`id`](QueryMembershipsMineResponseItemOrganizationBuilder::id)
    /// - [`image_url`](QueryMembershipsMineResponseItemOrganizationBuilder::image_url)
    /// - [`impersonation_granted`](QueryMembershipsMineResponseItemOrganizationBuilder::impersonation_granted)
    /// - [`name`](QueryMembershipsMineResponseItemOrganizationBuilder::name)
    /// - [`residency_policy`](QueryMembershipsMineResponseItemOrganizationBuilder::residency_policy)
    /// - [`slug`](QueryMembershipsMineResponseItemOrganizationBuilder::slug)
    /// - [`created_at`](QueryMembershipsMineResponseItemOrganizationBuilder::created_at)
    /// - [`updated_at`](QueryMembershipsMineResponseItemOrganizationBuilder::updated_at)
    pub fn build(self) -> Result<QueryMembershipsMineResponseItemOrganization, BuildError> {
        Ok(QueryMembershipsMineResponseItemOrganization {
            allow_member_workspace_creation: self.allow_member_workspace_creation.ok_or_else(|| BuildError::missing_field("allow_member_workspace_creation"))?,
            auto_assign_problems: self.auto_assign_problems.ok_or_else(|| BuildError::missing_field("auto_assign_problems"))?,
            auto_create_apps: self.auto_create_apps.ok_or_else(|| BuildError::missing_field("auto_create_apps"))?,
            clerk_org_id: self.clerk_org_id.ok_or_else(|| BuildError::missing_field("clerk_org_id"))?,
            clerk_updated_at: self.clerk_updated_at,
            data_residency_location: self.data_residency_location.ok_or_else(|| BuildError::missing_field("data_residency_location"))?,
            default_surface_id: self.default_surface_id,
            demo_reset_at: self.demo_reset_at,
            demo_reset_status: self.demo_reset_status,
            demo_workspace: self.demo_workspace.ok_or_else(|| BuildError::missing_field("demo_workspace"))?,
            enabled_auth_methods: self.enabled_auth_methods,
            environment_filtering: self.environment_filtering.ok_or_else(|| BuildError::missing_field("environment_filtering"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            image_url: self.image_url.ok_or_else(|| BuildError::missing_field("image_url"))?,
            impersonation_expires_at: self.impersonation_expires_at,
            impersonation_granted: self.impersonation_granted.ok_or_else(|| BuildError::missing_field("impersonation_granted"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            onboarding_completed_at: self.onboarding_completed_at,
            residency_policy: self.residency_policy.ok_or_else(|| BuildError::missing_field("residency_policy"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            stripe_customer_id: self.stripe_customer_id,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}

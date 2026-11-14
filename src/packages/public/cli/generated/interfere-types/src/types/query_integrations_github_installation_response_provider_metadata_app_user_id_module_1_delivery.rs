pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1Delivery {
    #[serde(rename = "providerWorkspaceId")]
    #[serde(default)]
    pub provider_workspace_id: String,
    #[serde(rename = "consentRef")]
    #[serde(default)]
    pub consent_ref: String,
    #[serde(rename = "consentedAt")]
    #[serde(default)]
    pub consented_at: i64,
    #[serde(rename = "verifiedTeams")]
    #[serde(default)]
    pub verified_teams: Vec<QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItem>,
}

impl QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1Delivery {
    pub fn builder() -> QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryBuilder {
        <QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryBuilder {
    provider_workspace_id: Option<String>,
    consent_ref: Option<String>,
    consented_at: Option<i64>,
    verified_teams: Option<Vec<QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItem>>,
}

impl QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryBuilder {
    pub fn provider_workspace_id(mut self, value: impl Into<String>) -> Self {
        self.provider_workspace_id = Some(value.into());
        self
    }

    pub fn consent_ref(mut self, value: impl Into<String>) -> Self {
        self.consent_ref = Some(value.into());
        self
    }

    pub fn consented_at(mut self, value: i64) -> Self {
        self.consented_at = Some(value);
        self
    }

    pub fn verified_teams(mut self, value: Vec<QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItem>) -> Self {
        self.verified_teams = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1Delivery`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider_workspace_id`](QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryBuilder::provider_workspace_id)
    /// - [`consent_ref`](QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryBuilder::consent_ref)
    /// - [`consented_at`](QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryBuilder::consented_at)
    /// - [`verified_teams`](QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryBuilder::verified_teams)
    pub fn build(self) -> Result<QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1Delivery, BuildError> {
        Ok(QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1Delivery {
            provider_workspace_id: self.provider_workspace_id.ok_or_else(|| BuildError::missing_field("provider_workspace_id"))?,
            consent_ref: self.consent_ref.ok_or_else(|| BuildError::missing_field("consent_ref"))?,
            consented_at: self.consented_at.ok_or_else(|| BuildError::missing_field("consented_at"))?,
            verified_teams: self.verified_teams.ok_or_else(|| BuildError::missing_field("verified_teams"))?,
        })
    }
}

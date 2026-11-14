pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItem {
    #[serde(rename = "teamId")]
    #[serde(default)]
    pub team_id: String,
    #[serde(rename = "verifiedAt")]
    #[serde(default)]
    pub verified_at: i64,
}

impl QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItem {
    pub fn builder() -> QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItemBuilder {
        <QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItemBuilder {
    team_id: Option<String>,
    verified_at: Option<i64>,
}

impl QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItemBuilder {
    pub fn team_id(mut self, value: impl Into<String>) -> Self {
        self.team_id = Some(value.into());
        self
    }

    pub fn verified_at(mut self, value: i64) -> Self {
        self.verified_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`team_id`](QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItemBuilder::team_id)
    /// - [`verified_at`](QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItemBuilder::verified_at)
    pub fn build(self) -> Result<QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItem, BuildError> {
        Ok(QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItem {
            team_id: self.team_id.ok_or_else(|| BuildError::missing_field("team_id"))?,
            verified_at: self.verified_at.ok_or_else(|| BuildError::missing_field("verified_at"))?,
        })
    }
}

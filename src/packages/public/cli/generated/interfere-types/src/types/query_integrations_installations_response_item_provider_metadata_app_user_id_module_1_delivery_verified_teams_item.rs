pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItem {
    #[serde(rename = "teamId")]
    #[serde(default)]
    pub team_id: String,
    #[serde(rename = "verifiedAt")]
    #[serde(default)]
    pub verified_at: i64,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItem {
    pub fn builder() -> QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItemBuilder {
        <QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItemBuilder {
    team_id: Option<String>,
    verified_at: Option<i64>,
}

impl QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItemBuilder {
    pub fn team_id(mut self, value: impl Into<String>) -> Self {
        self.team_id = Some(value.into());
        self
    }

    pub fn verified_at(mut self, value: i64) -> Self {
        self.verified_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`team_id`](QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItemBuilder::team_id)
    /// - [`verified_at`](QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItemBuilder::verified_at)
    pub fn build(self) -> Result<QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItem, BuildError> {
        Ok(QueryIntegrationsInstallationsResponseItemProviderMetadataAppUserIdModule1DeliveryVerifiedTeamsItem {
            team_id: self.team_id.ok_or_else(|| BuildError::missing_field("team_id"))?,
            verified_at: self.verified_at.ok_or_else(|| BuildError::missing_field("verified_at"))?,
        })
    }
}

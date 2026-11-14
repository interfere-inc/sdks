pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1 {
    #[serde(default)]
    pub version: String,
    pub mode: QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Mode,
    #[serde(default)]
    pub revision: i64,
    #[serde(rename = "authorizationGeneration")]
    #[serde(default)]
    pub authorization_generation: i64,
    #[serde(rename = "consentRef")]
    #[serde(default)]
    pub consent_ref: String,
    #[serde(rename = "createIssues")]
    #[serde(default)]
    pub create_issues: bool,
    #[serde(rename = "commentIssues")]
    #[serde(default)]
    pub comment_issues: bool,
    #[serde(rename = "destinationTeamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_team_id: Option<String>,
    #[serde(rename = "allowedTeamIds")]
    #[serde(default)]
    pub allowed_team_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Delivery>,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1 {
    pub fn builder() -> QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Builder {
        <QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Builder {
    version: Option<String>,
    mode: Option<QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Mode>,
    revision: Option<i64>,
    authorization_generation: Option<i64>,
    consent_ref: Option<String>,
    create_issues: Option<bool>,
    comment_issues: Option<bool>,
    destination_team_id: Option<String>,
    allowed_team_ids: Option<Vec<String>>,
    delivery: Option<QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Delivery>,
}

impl QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Builder {
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn mode(mut self, value: QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Mode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    pub fn authorization_generation(mut self, value: i64) -> Self {
        self.authorization_generation = Some(value);
        self
    }

    pub fn consent_ref(mut self, value: impl Into<String>) -> Self {
        self.consent_ref = Some(value.into());
        self
    }

    pub fn create_issues(mut self, value: bool) -> Self {
        self.create_issues = Some(value);
        self
    }

    pub fn comment_issues(mut self, value: bool) -> Self {
        self.comment_issues = Some(value);
        self
    }

    pub fn destination_team_id(mut self, value: impl Into<String>) -> Self {
        self.destination_team_id = Some(value.into());
        self
    }

    pub fn allowed_team_ids(mut self, value: Vec<String>) -> Self {
        self.allowed_team_ids = Some(value);
        self
    }

    pub fn delivery(mut self, value: QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Delivery) -> Self {
        self.delivery = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Builder::version)
    /// - [`mode`](QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Builder::mode)
    /// - [`revision`](QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Builder::revision)
    /// - [`authorization_generation`](QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Builder::authorization_generation)
    /// - [`consent_ref`](QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Builder::consent_ref)
    /// - [`create_issues`](QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Builder::create_issues)
    /// - [`comment_issues`](QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Builder::comment_issues)
    /// - [`allowed_team_ids`](QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1Builder::allowed_team_ids)
    pub fn build(self) -> Result<QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1, BuildError> {
        Ok(QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdModule1 {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
            authorization_generation: self.authorization_generation.ok_or_else(|| BuildError::missing_field("authorization_generation"))?,
            consent_ref: self.consent_ref.ok_or_else(|| BuildError::missing_field("consent_ref"))?,
            create_issues: self.create_issues.ok_or_else(|| BuildError::missing_field("create_issues"))?,
            comment_issues: self.comment_issues.ok_or_else(|| BuildError::missing_field("comment_issues"))?,
            destination_team_id: self.destination_team_id,
            allowed_team_ids: self.allowed_team_ids.ok_or_else(|| BuildError::missing_field("allowed_team_ids"))?,
            delivery: self.delivery,
        })
    }
}

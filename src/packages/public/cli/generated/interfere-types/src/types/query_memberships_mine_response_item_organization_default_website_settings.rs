pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettings {
    /// When sessions without an identified user record replay: always, only once an error occurs, or never.
    #[serde(rename = "anonymousReplayPolicy")]
    pub anonymous_replay_policy: QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettingsAnonymousReplayPolicy,
    #[serde(rename = "anonymousUserTracking")]
    #[serde(default)]
    pub anonymous_user_tracking: bool,
    #[serde(rename = "environmentFiltering")]
    #[serde(default)]
    pub environment_filtering: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettingsPlugins>,
}

impl QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettings {
    pub fn builder() -> QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettingsBuilder {
        <QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettingsBuilder {
    anonymous_replay_policy: Option<QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettingsAnonymousReplayPolicy>,
    anonymous_user_tracking: Option<bool>,
    environment_filtering: Option<bool>,
    plugins: Option<QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettingsPlugins>,
}

impl QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettingsBuilder {
    pub fn anonymous_replay_policy(mut self, value: QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettingsAnonymousReplayPolicy) -> Self {
        self.anonymous_replay_policy = Some(value);
        self
    }

    pub fn anonymous_user_tracking(mut self, value: bool) -> Self {
        self.anonymous_user_tracking = Some(value);
        self
    }

    pub fn environment_filtering(mut self, value: bool) -> Self {
        self.environment_filtering = Some(value);
        self
    }

    pub fn plugins(mut self, value: QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettingsPlugins) -> Self {
        self.plugins = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`anonymous_replay_policy`](QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettingsBuilder::anonymous_replay_policy)
    /// - [`anonymous_user_tracking`](QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettingsBuilder::anonymous_user_tracking)
    /// - [`environment_filtering`](QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettingsBuilder::environment_filtering)
    pub fn build(self) -> Result<QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettings, BuildError> {
        Ok(QueryMembershipsMineResponseItemOrganizationDefaultWebsiteSettings {
            anonymous_replay_policy: self.anonymous_replay_policy.ok_or_else(|| BuildError::missing_field("anonymous_replay_policy"))?,
            anonymous_user_tracking: self.anonymous_user_tracking.ok_or_else(|| BuildError::missing_field("anonymous_user_tracking"))?,
            environment_filtering: self.environment_filtering.ok_or_else(|| BuildError::missing_field("environment_filtering"))?,
            plugins: self.plugins,
        })
    }
}

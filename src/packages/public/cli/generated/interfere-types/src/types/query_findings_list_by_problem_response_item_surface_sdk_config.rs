pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryFindingsListByProblemResponseItemSurfaceSdkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<QueryFindingsListByProblemResponseItemSurfaceSdkConfigPlugins>,
    #[serde(rename = "serverTracing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_tracing: Option<QueryFindingsListByProblemResponseItemSurfaceSdkConfigServerTracing>,
    #[serde(rename = "logSourcing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_sourcing: Option<QueryFindingsListByProblemResponseItemSurfaceSdkConfigLogSourcing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<QueryFindingsListByProblemResponseItemSurfaceSdkConfigPlatform>,
}

impl QueryFindingsListByProblemResponseItemSurfaceSdkConfig {
    pub fn builder() -> QueryFindingsListByProblemResponseItemSurfaceSdkConfigBuilder {
        <QueryFindingsListByProblemResponseItemSurfaceSdkConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryFindingsListByProblemResponseItemSurfaceSdkConfigBuilder {
    plugins: Option<QueryFindingsListByProblemResponseItemSurfaceSdkConfigPlugins>,
    server_tracing: Option<QueryFindingsListByProblemResponseItemSurfaceSdkConfigServerTracing>,
    log_sourcing: Option<QueryFindingsListByProblemResponseItemSurfaceSdkConfigLogSourcing>,
    platform: Option<QueryFindingsListByProblemResponseItemSurfaceSdkConfigPlatform>,
}

impl QueryFindingsListByProblemResponseItemSurfaceSdkConfigBuilder {
    pub fn plugins(mut self, value: QueryFindingsListByProblemResponseItemSurfaceSdkConfigPlugins) -> Self {
        self.plugins = Some(value);
        self
    }

    pub fn server_tracing(mut self, value: QueryFindingsListByProblemResponseItemSurfaceSdkConfigServerTracing) -> Self {
        self.server_tracing = Some(value);
        self
    }

    pub fn log_sourcing(mut self, value: QueryFindingsListByProblemResponseItemSurfaceSdkConfigLogSourcing) -> Self {
        self.log_sourcing = Some(value);
        self
    }

    pub fn platform(mut self, value: QueryFindingsListByProblemResponseItemSurfaceSdkConfigPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryFindingsListByProblemResponseItemSurfaceSdkConfig`].
    pub fn build(self) -> Result<QueryFindingsListByProblemResponseItemSurfaceSdkConfig, BuildError> {
        Ok(QueryFindingsListByProblemResponseItemSurfaceSdkConfig {
            plugins: self.plugins,
            server_tracing: self.server_tracing,
            log_sourcing: self.log_sourcing,
            platform: self.platform,
        })
    }
}

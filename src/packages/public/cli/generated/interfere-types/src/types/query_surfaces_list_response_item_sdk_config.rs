pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuerySurfacesListResponseItemSdkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<QuerySurfacesListResponseItemSdkConfigPlugins>,
    #[serde(rename = "serverTracing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_tracing: Option<QuerySurfacesListResponseItemSdkConfigServerTracing>,
    #[serde(rename = "logSourcing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_sourcing: Option<QuerySurfacesListResponseItemSdkConfigLogSourcing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<QuerySurfacesListResponseItemSdkConfigPlatform>,
}

impl QuerySurfacesListResponseItemSdkConfig {
    pub fn builder() -> QuerySurfacesListResponseItemSdkConfigBuilder {
        <QuerySurfacesListResponseItemSdkConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySurfacesListResponseItemSdkConfigBuilder {
    plugins: Option<QuerySurfacesListResponseItemSdkConfigPlugins>,
    server_tracing: Option<QuerySurfacesListResponseItemSdkConfigServerTracing>,
    log_sourcing: Option<QuerySurfacesListResponseItemSdkConfigLogSourcing>,
    platform: Option<QuerySurfacesListResponseItemSdkConfigPlatform>,
}

impl QuerySurfacesListResponseItemSdkConfigBuilder {
    pub fn plugins(mut self, value: QuerySurfacesListResponseItemSdkConfigPlugins) -> Self {
        self.plugins = Some(value);
        self
    }

    pub fn server_tracing(mut self, value: QuerySurfacesListResponseItemSdkConfigServerTracing) -> Self {
        self.server_tracing = Some(value);
        self
    }

    pub fn log_sourcing(mut self, value: QuerySurfacesListResponseItemSdkConfigLogSourcing) -> Self {
        self.log_sourcing = Some(value);
        self
    }

    pub fn platform(mut self, value: QuerySurfacesListResponseItemSdkConfigPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuerySurfacesListResponseItemSdkConfig`].
    pub fn build(self) -> Result<QuerySurfacesListResponseItemSdkConfig, BuildError> {
        Ok(QuerySurfacesListResponseItemSdkConfig {
            plugins: self.plugins,
            server_tracing: self.server_tracing,
            log_sourcing: self.log_sourcing,
            platform: self.platform,
        })
    }
}

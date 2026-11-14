pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigPlugins>,
    #[serde(rename = "serverTracing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_tracing: Option<QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigServerTracing>,
    #[serde(rename = "logSourcing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_sourcing: Option<QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigLogSourcing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigPlatform>,
}

impl QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfig {
    pub fn builder() -> QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigBuilder {
        <QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigBuilder {
    plugins: Option<QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigPlugins>,
    server_tracing: Option<QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigServerTracing>,
    log_sourcing: Option<QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigLogSourcing>,
    platform: Option<QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigPlatform>,
}

impl QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigBuilder {
    pub fn plugins(mut self, value: QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigPlugins) -> Self {
        self.plugins = Some(value);
        self
    }

    pub fn server_tracing(mut self, value: QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigServerTracing) -> Self {
        self.server_tracing = Some(value);
        self
    }

    pub fn log_sourcing(mut self, value: QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigLogSourcing) -> Self {
        self.log_sourcing = Some(value);
        self
    }

    pub fn platform(mut self, value: QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfigPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfig`].
    pub fn build(self) -> Result<QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfig, BuildError> {
        Ok(QuerySurfacesGetBySlugIncludeDeletedResponseSdkConfig {
            plugins: self.plugins,
            server_tracing: self.server_tracing,
            log_sourcing: self.log_sourcing,
            platform: self.platform,
        })
    }
}

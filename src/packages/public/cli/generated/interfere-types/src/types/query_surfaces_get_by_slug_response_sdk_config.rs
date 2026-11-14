pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuerySurfacesGetBySlugResponseSdkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<QuerySurfacesGetBySlugResponseSdkConfigPlugins>,
    #[serde(rename = "serverTracing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_tracing: Option<QuerySurfacesGetBySlugResponseSdkConfigServerTracing>,
    #[serde(rename = "logSourcing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_sourcing: Option<QuerySurfacesGetBySlugResponseSdkConfigLogSourcing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<QuerySurfacesGetBySlugResponseSdkConfigPlatform>,
}

impl QuerySurfacesGetBySlugResponseSdkConfig {
    pub fn builder() -> QuerySurfacesGetBySlugResponseSdkConfigBuilder {
        <QuerySurfacesGetBySlugResponseSdkConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySurfacesGetBySlugResponseSdkConfigBuilder {
    plugins: Option<QuerySurfacesGetBySlugResponseSdkConfigPlugins>,
    server_tracing: Option<QuerySurfacesGetBySlugResponseSdkConfigServerTracing>,
    log_sourcing: Option<QuerySurfacesGetBySlugResponseSdkConfigLogSourcing>,
    platform: Option<QuerySurfacesGetBySlugResponseSdkConfigPlatform>,
}

impl QuerySurfacesGetBySlugResponseSdkConfigBuilder {
    pub fn plugins(mut self, value: QuerySurfacesGetBySlugResponseSdkConfigPlugins) -> Self {
        self.plugins = Some(value);
        self
    }

    pub fn server_tracing(mut self, value: QuerySurfacesGetBySlugResponseSdkConfigServerTracing) -> Self {
        self.server_tracing = Some(value);
        self
    }

    pub fn log_sourcing(mut self, value: QuerySurfacesGetBySlugResponseSdkConfigLogSourcing) -> Self {
        self.log_sourcing = Some(value);
        self
    }

    pub fn platform(mut self, value: QuerySurfacesGetBySlugResponseSdkConfigPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuerySurfacesGetBySlugResponseSdkConfig`].
    pub fn build(self) -> Result<QuerySurfacesGetBySlugResponseSdkConfig, BuildError> {
        Ok(QuerySurfacesGetBySlugResponseSdkConfig {
            plugins: self.plugins,
            server_tracing: self.server_tracing,
            log_sourcing: self.log_sourcing,
            platform: self.platform,
        })
    }
}

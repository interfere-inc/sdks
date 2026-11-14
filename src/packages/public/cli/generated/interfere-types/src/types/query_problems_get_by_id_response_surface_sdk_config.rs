pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseSurfaceSdkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<QueryProblemsGetByIdResponseSurfaceSdkConfigPlugins>,
    #[serde(rename = "serverTracing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_tracing: Option<QueryProblemsGetByIdResponseSurfaceSdkConfigServerTracing>,
    #[serde(rename = "logSourcing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_sourcing: Option<QueryProblemsGetByIdResponseSurfaceSdkConfigLogSourcing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<QueryProblemsGetByIdResponseSurfaceSdkConfigPlatform>,
}

impl QueryProblemsGetByIdResponseSurfaceSdkConfig {
    pub fn builder() -> QueryProblemsGetByIdResponseSurfaceSdkConfigBuilder {
        <QueryProblemsGetByIdResponseSurfaceSdkConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseSurfaceSdkConfigBuilder {
    plugins: Option<QueryProblemsGetByIdResponseSurfaceSdkConfigPlugins>,
    server_tracing: Option<QueryProblemsGetByIdResponseSurfaceSdkConfigServerTracing>,
    log_sourcing: Option<QueryProblemsGetByIdResponseSurfaceSdkConfigLogSourcing>,
    platform: Option<QueryProblemsGetByIdResponseSurfaceSdkConfigPlatform>,
}

impl QueryProblemsGetByIdResponseSurfaceSdkConfigBuilder {
    pub fn plugins(mut self, value: QueryProblemsGetByIdResponseSurfaceSdkConfigPlugins) -> Self {
        self.plugins = Some(value);
        self
    }

    pub fn server_tracing(mut self, value: QueryProblemsGetByIdResponseSurfaceSdkConfigServerTracing) -> Self {
        self.server_tracing = Some(value);
        self
    }

    pub fn log_sourcing(mut self, value: QueryProblemsGetByIdResponseSurfaceSdkConfigLogSourcing) -> Self {
        self.log_sourcing = Some(value);
        self
    }

    pub fn platform(mut self, value: QueryProblemsGetByIdResponseSurfaceSdkConfigPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseSurfaceSdkConfig`].
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseSurfaceSdkConfig, BuildError> {
        Ok(QueryProblemsGetByIdResponseSurfaceSdkConfig {
            plugins: self.plugins,
            server_tracing: self.server_tracing,
            log_sourcing: self.log_sourcing,
            platform: self.platform,
        })
    }
}

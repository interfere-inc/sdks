pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReleasesConfigResponseSurfaceSdkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<ReleasesConfigResponseSurfaceSdkConfigPlugins>,
    #[serde(rename = "serverTracing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_tracing: Option<ReleasesConfigResponseSurfaceSdkConfigServerTracing>,
    #[serde(rename = "logSourcing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_sourcing: Option<ReleasesConfigResponseSurfaceSdkConfigLogSourcing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<ReleasesConfigResponseSurfaceSdkConfigPlatform>,
}

impl ReleasesConfigResponseSurfaceSdkConfig {
    pub fn builder() -> ReleasesConfigResponseSurfaceSdkConfigBuilder {
        <ReleasesConfigResponseSurfaceSdkConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReleasesConfigResponseSurfaceSdkConfigBuilder {
    plugins: Option<ReleasesConfigResponseSurfaceSdkConfigPlugins>,
    server_tracing: Option<ReleasesConfigResponseSurfaceSdkConfigServerTracing>,
    log_sourcing: Option<ReleasesConfigResponseSurfaceSdkConfigLogSourcing>,
    platform: Option<ReleasesConfigResponseSurfaceSdkConfigPlatform>,
}

impl ReleasesConfigResponseSurfaceSdkConfigBuilder {
    pub fn plugins(mut self, value: ReleasesConfigResponseSurfaceSdkConfigPlugins) -> Self {
        self.plugins = Some(value);
        self
    }

    pub fn server_tracing(mut self, value: ReleasesConfigResponseSurfaceSdkConfigServerTracing) -> Self {
        self.server_tracing = Some(value);
        self
    }

    pub fn log_sourcing(mut self, value: ReleasesConfigResponseSurfaceSdkConfigLogSourcing) -> Self {
        self.log_sourcing = Some(value);
        self
    }

    pub fn platform(mut self, value: ReleasesConfigResponseSurfaceSdkConfigPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReleasesConfigResponseSurfaceSdkConfig`].
    pub fn build(self) -> Result<ReleasesConfigResponseSurfaceSdkConfig, BuildError> {
        Ok(ReleasesConfigResponseSurfaceSdkConfig {
            plugins: self.plugins,
            server_tracing: self.server_tracing,
            log_sourcing: self.log_sourcing,
            platform: self.platform,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigPlugins>,
    #[serde(rename = "serverTracing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_tracing: Option<QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigServerTracing>,
    #[serde(rename = "logSourcing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_sourcing: Option<QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigLogSourcing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigPlatform>,
}

impl QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfig {
    pub fn builder() -> QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigBuilder {
        <QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigBuilder {
    plugins: Option<QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigPlugins>,
    server_tracing: Option<QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigServerTracing>,
    log_sourcing: Option<QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigLogSourcing>,
    platform: Option<QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigPlatform>,
}

impl QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigBuilder {
    pub fn plugins(mut self, value: QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigPlugins) -> Self {
        self.plugins = Some(value);
        self
    }

    pub fn server_tracing(mut self, value: QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigServerTracing) -> Self {
        self.server_tracing = Some(value);
        self
    }

    pub fn log_sourcing(mut self, value: QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigLogSourcing) -> Self {
        self.log_sourcing = Some(value);
        self
    }

    pub fn platform(mut self, value: QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfigPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfig`].
    pub fn build(self) -> Result<QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfig, BuildError> {
        Ok(QueryConsumerDevicesByEndUserResponseItemSurfaceSdkConfig {
            plugins: self.plugins,
            server_tracing: self.server_tracing,
            log_sourcing: self.log_sourcing,
            platform: self.platform,
        })
    }
}

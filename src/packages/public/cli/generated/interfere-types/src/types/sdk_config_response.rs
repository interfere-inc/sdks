pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Runtime plugin and sourcing config for the browser SDK.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SdkConfigResponse {
    /// When sessions without an identified user record replay: always, only once an error occurs, or never.
    #[serde(rename = "anonymousReplayPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_replay_policy: Option<SdkConfigResponseAnonymousReplayPolicy>,
    #[serde(rename = "backendHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_hosts: Option<Vec<String>>,
    #[serde(rename = "ingestUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_url: Option<String>,
    #[serde(rename = "logSourcing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_sourcing: Option<SdkConfigResponseLogSourcing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<SdkConfigResponsePlatform>,
    #[serde(default)]
    pub plugins: HashMap<String, bool>,
    #[serde(rename = "protocolVersion")]
    #[serde(default)]
    pub protocol_version: CaptureProtocolVersion,
    #[serde(rename = "serverTracing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_tracing: Option<SdkConfigResponseServerTracing>,
}

impl SdkConfigResponse {
    pub fn builder() -> SdkConfigResponseBuilder {
        <SdkConfigResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SdkConfigResponseBuilder {
    anonymous_replay_policy: Option<SdkConfigResponseAnonymousReplayPolicy>,
    backend_hosts: Option<Vec<String>>,
    ingest_url: Option<String>,
    log_sourcing: Option<SdkConfigResponseLogSourcing>,
    platform: Option<SdkConfigResponsePlatform>,
    plugins: Option<HashMap<String, bool>>,
    protocol_version: Option<CaptureProtocolVersion>,
    server_tracing: Option<SdkConfigResponseServerTracing>,
}

impl SdkConfigResponseBuilder {
    pub fn anonymous_replay_policy(mut self, value: SdkConfigResponseAnonymousReplayPolicy) -> Self {
        self.anonymous_replay_policy = Some(value);
        self
    }

    pub fn backend_hosts(mut self, value: Vec<String>) -> Self {
        self.backend_hosts = Some(value);
        self
    }

    pub fn ingest_url(mut self, value: impl Into<String>) -> Self {
        self.ingest_url = Some(value.into());
        self
    }

    pub fn log_sourcing(mut self, value: SdkConfigResponseLogSourcing) -> Self {
        self.log_sourcing = Some(value);
        self
    }

    pub fn platform(mut self, value: SdkConfigResponsePlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn plugins(mut self, value: HashMap<String, bool>) -> Self {
        self.plugins = Some(value);
        self
    }

    pub fn protocol_version(mut self, value: CaptureProtocolVersion) -> Self {
        self.protocol_version = Some(value);
        self
    }

    pub fn server_tracing(mut self, value: SdkConfigResponseServerTracing) -> Self {
        self.server_tracing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SdkConfigResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`plugins`](SdkConfigResponseBuilder::plugins)
    /// - [`protocol_version`](SdkConfigResponseBuilder::protocol_version)
    pub fn build(self) -> Result<SdkConfigResponse, BuildError> {
        Ok(SdkConfigResponse {
            anonymous_replay_policy: self.anonymous_replay_policy,
            backend_hosts: self.backend_hosts,
            ingest_url: self.ingest_url,
            log_sourcing: self.log_sourcing,
            platform: self.platform,
            plugins: self.plugins.ok_or_else(|| BuildError::missing_field("plugins"))?,
            protocol_version: self.protocol_version.ok_or_else(|| BuildError::missing_field("protocol_version"))?,
            server_tracing: self.server_tracing,
        })
    }
}

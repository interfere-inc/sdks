pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateSurfaceRequest {
    /// When sessions without an identified user record replay. Defaults to on_error when anonymous-user tracking is on, otherwise always.
    #[serde(rename = "anonymousReplayPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_replay_policy: Option<CreateSurfaceRequestAnonymousReplayPolicy>,
    /// Whether anonymous-user telemetry is collected.
    #[serde(rename = "anonymousUserTracking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_user_tracking: Option<bool>,
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: CreateSurfaceRequestApiKey,
    #[serde(rename = "idempotencyKey")]
    #[serde(default)]
    pub idempotency_key: String,
    /// Display name of the surface.
    #[serde(default)]
    pub name: String,
    /// Application framework used by the surface.
    pub r#type: CreateSurfaceRequestType,
}

impl CreateSurfaceRequest {
    pub fn builder() -> CreateSurfaceRequestBuilder {
        <CreateSurfaceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSurfaceRequestBuilder {
    anonymous_replay_policy: Option<CreateSurfaceRequestAnonymousReplayPolicy>,
    anonymous_user_tracking: Option<bool>,
    api_key: Option<CreateSurfaceRequestApiKey>,
    idempotency_key: Option<String>,
    name: Option<String>,
    r#type: Option<CreateSurfaceRequestType>,
}

impl CreateSurfaceRequestBuilder {
    pub fn anonymous_replay_policy(mut self, value: CreateSurfaceRequestAnonymousReplayPolicy) -> Self {
        self.anonymous_replay_policy = Some(value);
        self
    }

    pub fn anonymous_user_tracking(mut self, value: bool) -> Self {
        self.anonymous_user_tracking = Some(value);
        self
    }

    pub fn api_key(mut self, value: CreateSurfaceRequestApiKey) -> Self {
        self.api_key = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: CreateSurfaceRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSurfaceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CreateSurfaceRequestBuilder::api_key)
    /// - [`idempotency_key`](CreateSurfaceRequestBuilder::idempotency_key)
    /// - [`name`](CreateSurfaceRequestBuilder::name)
    /// - [`r#type`](CreateSurfaceRequestBuilder::r#type)
    pub fn build(self) -> Result<CreateSurfaceRequest, BuildError> {
        Ok(CreateSurfaceRequest {
            anonymous_replay_policy: self.anonymous_replay_policy,
            anonymous_user_tracking: self.anonymous_user_tracking,
            api_key: self.api_key.ok_or_else(|| BuildError::missing_field("api_key"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}


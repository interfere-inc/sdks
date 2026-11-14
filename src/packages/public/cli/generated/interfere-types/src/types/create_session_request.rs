pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSessionRequest {
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "fpHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fp_hash: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
    #[serde(rename = "visitorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visitor_id: Option<String>,
    /// Why the SDK started recording an anonymous session: error marks the buffer flushed when the visitor hit an error.
    #[serde(rename = "replayTrigger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_trigger: Option<CreateSessionRequestReplayTrigger>,
}

impl CreateSessionRequest {
    pub fn builder() -> CreateSessionRequestBuilder {
        <CreateSessionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSessionRequestBuilder {
    device_id: Option<String>,
    fp_hash: Option<String>,
    session_id: Option<String>,
    visitor_id: Option<String>,
    replay_trigger: Option<CreateSessionRequestReplayTrigger>,
}

impl CreateSessionRequestBuilder {
    pub fn device_id(mut self, value: impl Into<String>) -> Self {
        self.device_id = Some(value.into());
        self
    }

    pub fn fp_hash(mut self, value: impl Into<String>) -> Self {
        self.fp_hash = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn visitor_id(mut self, value: impl Into<String>) -> Self {
        self.visitor_id = Some(value.into());
        self
    }

    pub fn replay_trigger(mut self, value: CreateSessionRequestReplayTrigger) -> Self {
        self.replay_trigger = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSessionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`session_id`](CreateSessionRequestBuilder::session_id)
    pub fn build(self) -> Result<CreateSessionRequest, BuildError> {
        Ok(CreateSessionRequest {
            device_id: self.device_id,
            fp_hash: self.fp_hash,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            visitor_id: self.visitor_id,
            replay_trigger: self.replay_trigger,
        })
    }
}


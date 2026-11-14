pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// RpcStatus
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RpcStatus {
    /// google.rpc.Code value mirroring the HTTP status
    #[serde(default)]
    pub code: i64,
    #[serde(default)]
    pub message: String,
}

impl RpcStatus {
    pub fn builder() -> RpcStatusBuilder {
        <RpcStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RpcStatusBuilder {
    code: Option<i64>,
    message: Option<String>,
}

impl RpcStatusBuilder {
    pub fn code(mut self, value: i64) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RpcStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](RpcStatusBuilder::code)
    /// - [`message`](RpcStatusBuilder::message)
    pub fn build(self) -> Result<RpcStatus, BuildError> {
        Ok(RpcStatus {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}

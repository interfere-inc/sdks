pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RpcUnexpectedError {
    #[serde(default)]
    pub defined: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub status: f64,
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl RpcUnexpectedError {
    pub fn builder() -> RpcUnexpectedErrorBuilder {
        <RpcUnexpectedErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RpcUnexpectedErrorBuilder {
    defined: Option<String>,
    code: Option<String>,
    status: Option<f64>,
    message: Option<String>,
    data: Option<serde_json::Value>,
}

impl RpcUnexpectedErrorBuilder {
    pub fn defined(mut self, value: impl Into<String>) -> Self {
        self.defined = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn status(mut self, value: f64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn data(mut self, value: serde_json::Value) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RpcUnexpectedError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`defined`](RpcUnexpectedErrorBuilder::defined)
    /// - [`code`](RpcUnexpectedErrorBuilder::code)
    /// - [`status`](RpcUnexpectedErrorBuilder::status)
    /// - [`message`](RpcUnexpectedErrorBuilder::message)
    pub fn build(self) -> Result<RpcUnexpectedError, BuildError> {
        Ok(RpcUnexpectedError {
            defined: self.defined.ok_or_else(|| BuildError::missing_field("defined"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            data: self.data,
        })
    }
}

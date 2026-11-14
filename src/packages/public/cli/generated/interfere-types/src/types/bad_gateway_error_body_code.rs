pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BadGatewayErrorBodyCode {
    #[serde(default)]
    pub defined: String,
    pub code: BadGatewayErrorBodyCodeCode,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl BadGatewayErrorBodyCode {
    pub fn builder() -> BadGatewayErrorBodyCodeBuilder {
        <BadGatewayErrorBodyCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BadGatewayErrorBodyCodeBuilder {
    defined: Option<String>,
    code: Option<BadGatewayErrorBodyCodeCode>,
    status: Option<String>,
    message: Option<String>,
    data: Option<serde_json::Value>,
}

impl BadGatewayErrorBodyCodeBuilder {
    pub fn defined(mut self, value: impl Into<String>) -> Self {
        self.defined = Some(value.into());
        self
    }

    pub fn code(mut self, value: BadGatewayErrorBodyCodeCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
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

    /// Consumes the builder and constructs a [`BadGatewayErrorBodyCode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`defined`](BadGatewayErrorBodyCodeBuilder::defined)
    /// - [`code`](BadGatewayErrorBodyCodeBuilder::code)
    /// - [`status`](BadGatewayErrorBodyCodeBuilder::status)
    /// - [`message`](BadGatewayErrorBodyCodeBuilder::message)
    pub fn build(self) -> Result<BadGatewayErrorBodyCode, BuildError> {
        Ok(BadGatewayErrorBodyCode {
            defined: self.defined.ok_or_else(|| BuildError::missing_field("defined"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            data: self.data,
        })
    }
}

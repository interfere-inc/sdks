pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BadRequestErrorBodyCode {
    #[serde(default)]
    pub defined: String,
    pub code: BadRequestErrorBodyCodeCode,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl BadRequestErrorBodyCode {
    pub fn builder() -> BadRequestErrorBodyCodeBuilder {
        <BadRequestErrorBodyCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BadRequestErrorBodyCodeBuilder {
    defined: Option<String>,
    code: Option<BadRequestErrorBodyCodeCode>,
    status: Option<String>,
    message: Option<String>,
    data: Option<serde_json::Value>,
}

impl BadRequestErrorBodyCodeBuilder {
    pub fn defined(mut self, value: impl Into<String>) -> Self {
        self.defined = Some(value.into());
        self
    }

    pub fn code(mut self, value: BadRequestErrorBodyCodeCode) -> Self {
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

    /// Consumes the builder and constructs a [`BadRequestErrorBodyCode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`defined`](BadRequestErrorBodyCodeBuilder::defined)
    /// - [`code`](BadRequestErrorBodyCodeBuilder::code)
    /// - [`status`](BadRequestErrorBodyCodeBuilder::status)
    /// - [`message`](BadRequestErrorBodyCodeBuilder::message)
    pub fn build(self) -> Result<BadRequestErrorBodyCode, BuildError> {
        Ok(BadRequestErrorBodyCode {
            defined: self.defined.ok_or_else(|| BuildError::missing_field("defined"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            data: self.data,
        })
    }
}

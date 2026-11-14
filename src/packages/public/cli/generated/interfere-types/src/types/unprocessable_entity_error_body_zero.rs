pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnprocessableEntityErrorBodyZero {
    #[serde(default)]
    pub defined: String,
    pub code: UnprocessableEntityErrorBodyZeroCode,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl UnprocessableEntityErrorBodyZero {
    pub fn builder() -> UnprocessableEntityErrorBodyZeroBuilder {
        <UnprocessableEntityErrorBodyZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnprocessableEntityErrorBodyZeroBuilder {
    defined: Option<String>,
    code: Option<UnprocessableEntityErrorBodyZeroCode>,
    status: Option<String>,
    message: Option<String>,
    data: Option<serde_json::Value>,
}

impl UnprocessableEntityErrorBodyZeroBuilder {
    pub fn defined(mut self, value: impl Into<String>) -> Self {
        self.defined = Some(value.into());
        self
    }

    pub fn code(mut self, value: UnprocessableEntityErrorBodyZeroCode) -> Self {
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

    /// Consumes the builder and constructs a [`UnprocessableEntityErrorBodyZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`defined`](UnprocessableEntityErrorBodyZeroBuilder::defined)
    /// - [`code`](UnprocessableEntityErrorBodyZeroBuilder::code)
    /// - [`status`](UnprocessableEntityErrorBodyZeroBuilder::status)
    /// - [`message`](UnprocessableEntityErrorBodyZeroBuilder::message)
    pub fn build(self) -> Result<UnprocessableEntityErrorBodyZero, BuildError> {
        Ok(UnprocessableEntityErrorBodyZero {
            defined: self.defined.ok_or_else(|| BuildError::missing_field("defined"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            data: self.data,
        })
    }
}

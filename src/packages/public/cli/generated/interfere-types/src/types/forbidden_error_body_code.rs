pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForbiddenErrorBodyCode {
    #[serde(default)]
    pub defined: String,
    pub code: ForbiddenErrorBodyCodeCode,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl ForbiddenErrorBodyCode {
    pub fn builder() -> ForbiddenErrorBodyCodeBuilder {
        <ForbiddenErrorBodyCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForbiddenErrorBodyCodeBuilder {
    defined: Option<String>,
    code: Option<ForbiddenErrorBodyCodeCode>,
    status: Option<String>,
    message: Option<String>,
    data: Option<serde_json::Value>,
}

impl ForbiddenErrorBodyCodeBuilder {
    pub fn defined(mut self, value: impl Into<String>) -> Self {
        self.defined = Some(value.into());
        self
    }

    pub fn code(mut self, value: ForbiddenErrorBodyCodeCode) -> Self {
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

    /// Consumes the builder and constructs a [`ForbiddenErrorBodyCode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`defined`](ForbiddenErrorBodyCodeBuilder::defined)
    /// - [`code`](ForbiddenErrorBodyCodeBuilder::code)
    /// - [`status`](ForbiddenErrorBodyCodeBuilder::status)
    /// - [`message`](ForbiddenErrorBodyCodeBuilder::message)
    pub fn build(self) -> Result<ForbiddenErrorBodyCode, BuildError> {
        Ok(ForbiddenErrorBodyCode {
            defined: self.defined.ok_or_else(|| BuildError::missing_field("defined"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            data: self.data,
        })
    }
}

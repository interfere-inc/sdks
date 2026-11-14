pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NotFoundErrorBodyOne {
    #[serde(default)]
    pub defined: String,
    pub code: NotFoundErrorBodyOneCode,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl NotFoundErrorBodyOne {
    pub fn builder() -> NotFoundErrorBodyOneBuilder {
        <NotFoundErrorBodyOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotFoundErrorBodyOneBuilder {
    defined: Option<String>,
    code: Option<NotFoundErrorBodyOneCode>,
    status: Option<String>,
    message: Option<String>,
    data: Option<serde_json::Value>,
}

impl NotFoundErrorBodyOneBuilder {
    pub fn defined(mut self, value: impl Into<String>) -> Self {
        self.defined = Some(value.into());
        self
    }

    pub fn code(mut self, value: NotFoundErrorBodyOneCode) -> Self {
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

    /// Consumes the builder and constructs a [`NotFoundErrorBodyOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`defined`](NotFoundErrorBodyOneBuilder::defined)
    /// - [`code`](NotFoundErrorBodyOneBuilder::code)
    /// - [`status`](NotFoundErrorBodyOneBuilder::status)
    /// - [`message`](NotFoundErrorBodyOneBuilder::message)
    pub fn build(self) -> Result<NotFoundErrorBodyOne, BuildError> {
        Ok(NotFoundErrorBodyOne {
            defined: self.defined.ok_or_else(|| BuildError::missing_field("defined"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            data: self.data,
        })
    }
}

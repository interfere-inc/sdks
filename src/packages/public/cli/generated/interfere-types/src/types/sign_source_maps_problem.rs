pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SignSourceMapsProblem {
    pub code: SignSourceMapsProblemCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<SignSourceMapsProblemService>,
}

impl SignSourceMapsProblem {
    pub fn builder() -> SignSourceMapsProblemBuilder {
        <SignSourceMapsProblemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignSourceMapsProblemBuilder {
    code: Option<SignSourceMapsProblemCode>,
    message: Option<String>,
    service: Option<SignSourceMapsProblemService>,
}

impl SignSourceMapsProblemBuilder {
    pub fn code(mut self, value: SignSourceMapsProblemCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn service(mut self, value: SignSourceMapsProblemService) -> Self {
        self.service = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SignSourceMapsProblem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](SignSourceMapsProblemBuilder::code)
    pub fn build(self) -> Result<SignSourceMapsProblem, BuildError> {
        Ok(SignSourceMapsProblem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message,
            service: self.service,
        })
    }
}

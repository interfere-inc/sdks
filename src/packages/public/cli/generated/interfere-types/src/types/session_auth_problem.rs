pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SessionAuthProblem {
    pub code: SessionAuthProblemCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<SessionAuthProblemService>,
}

impl SessionAuthProblem {
    pub fn builder() -> SessionAuthProblemBuilder {
        <SessionAuthProblemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionAuthProblemBuilder {
    code: Option<SessionAuthProblemCode>,
    message: Option<String>,
    service: Option<SessionAuthProblemService>,
}

impl SessionAuthProblemBuilder {
    pub fn code(mut self, value: SessionAuthProblemCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn service(mut self, value: SessionAuthProblemService) -> Self {
        self.service = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionAuthProblem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](SessionAuthProblemBuilder::code)
    pub fn build(self) -> Result<SessionAuthProblem, BuildError> {
        Ok(SessionAuthProblem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message,
            service: self.service,
        })
    }
}

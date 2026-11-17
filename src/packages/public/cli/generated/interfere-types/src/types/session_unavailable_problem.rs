pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SessionUnavailableProblem {
    pub code: SessionUnavailableProblemCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ErrorService>,
}

impl SessionUnavailableProblem {
    pub fn builder() -> SessionUnavailableProblemBuilder {
        <SessionUnavailableProblemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionUnavailableProblemBuilder {
    code: Option<SessionUnavailableProblemCode>,
    message: Option<String>,
    service: Option<ErrorService>,
}

impl SessionUnavailableProblemBuilder {
    pub fn code(mut self, value: SessionUnavailableProblemCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn service(mut self, value: ErrorService) -> Self {
        self.service = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionUnavailableProblem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](SessionUnavailableProblemBuilder::code)
    pub fn build(self) -> Result<SessionUnavailableProblem, BuildError> {
        Ok(SessionUnavailableProblem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message,
            service: self.service,
        })
    }
}

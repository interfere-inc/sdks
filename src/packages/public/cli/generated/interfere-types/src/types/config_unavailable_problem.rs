pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConfigUnavailableProblem {
    pub code: ConfigUnavailableProblemCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ConfigUnavailableProblemService>,
}

impl ConfigUnavailableProblem {
    pub fn builder() -> ConfigUnavailableProblemBuilder {
        <ConfigUnavailableProblemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigUnavailableProblemBuilder {
    code: Option<ConfigUnavailableProblemCode>,
    message: Option<String>,
    service: Option<ConfigUnavailableProblemService>,
}

impl ConfigUnavailableProblemBuilder {
    pub fn code(mut self, value: ConfigUnavailableProblemCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn service(mut self, value: ConfigUnavailableProblemService) -> Self {
        self.service = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfigUnavailableProblem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ConfigUnavailableProblemBuilder::code)
    pub fn build(self) -> Result<ConfigUnavailableProblem, BuildError> {
        Ok(ConfigUnavailableProblem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message,
            service: self.service,
        })
    }
}

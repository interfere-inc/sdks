pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FindingOccurrencesUnavailableProblemZero {
    pub code: FindingOccurrencesUnavailableProblemZeroCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ErrorService>,
}

impl FindingOccurrencesUnavailableProblemZero {
    pub fn builder() -> FindingOccurrencesUnavailableProblemZeroBuilder {
        <FindingOccurrencesUnavailableProblemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FindingOccurrencesUnavailableProblemZeroBuilder {
    code: Option<FindingOccurrencesUnavailableProblemZeroCode>,
    message: Option<String>,
    service: Option<ErrorService>,
}

impl FindingOccurrencesUnavailableProblemZeroBuilder {
    pub fn code(mut self, value: FindingOccurrencesUnavailableProblemZeroCode) -> Self {
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

    /// Consumes the builder and constructs a [`FindingOccurrencesUnavailableProblemZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](FindingOccurrencesUnavailableProblemZeroBuilder::code)
    pub fn build(self) -> Result<FindingOccurrencesUnavailableProblemZero, BuildError> {
        Ok(FindingOccurrencesUnavailableProblemZero {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message,
            service: self.service,
        })
    }
}

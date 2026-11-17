pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FindingOccurrencesUnavailableProblemCode {
    pub code: FindingOccurrencesUnavailableProblemCodeCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ErrorService>,
}

impl FindingOccurrencesUnavailableProblemCode {
    pub fn builder() -> FindingOccurrencesUnavailableProblemCodeBuilder {
        <FindingOccurrencesUnavailableProblemCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FindingOccurrencesUnavailableProblemCodeBuilder {
    code: Option<FindingOccurrencesUnavailableProblemCodeCode>,
    message: Option<String>,
    service: Option<ErrorService>,
}

impl FindingOccurrencesUnavailableProblemCodeBuilder {
    pub fn code(mut self, value: FindingOccurrencesUnavailableProblemCodeCode) -> Self {
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

    /// Consumes the builder and constructs a [`FindingOccurrencesUnavailableProblemCode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](FindingOccurrencesUnavailableProblemCodeBuilder::code)
    pub fn build(self) -> Result<FindingOccurrencesUnavailableProblemCode, BuildError> {
        Ok(FindingOccurrencesUnavailableProblemCode {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message,
            service: self.service,
        })
    }
}

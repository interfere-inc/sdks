pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FindingOccurrences {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default)]
    pub traces: Vec<FindingOccurrence>,
}

impl FindingOccurrences {
    pub fn builder() -> FindingOccurrencesBuilder {
        <FindingOccurrencesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FindingOccurrencesBuilder {
    cursor: Option<String>,
    traces: Option<Vec<FindingOccurrence>>,
}

impl FindingOccurrencesBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn traces(mut self, value: Vec<FindingOccurrence>) -> Self {
        self.traces = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FindingOccurrences`].
    /// This method will fail if any of the following fields are not set:
    /// - [`traces`](FindingOccurrencesBuilder::traces)
    pub fn build(self) -> Result<FindingOccurrences, BuildError> {
        Ok(FindingOccurrences {
            cursor: self.cursor,
            traces: self.traces.ok_or_else(|| BuildError::missing_field("traces"))?,
        })
    }
}

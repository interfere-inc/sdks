pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CorrectRootCauseRequestCauseLocationsItemLines {
    #[serde(default)]
    pub start: i64,
    #[serde(default)]
    pub end: i64,
}

impl CorrectRootCauseRequestCauseLocationsItemLines {
    pub fn builder() -> CorrectRootCauseRequestCauseLocationsItemLinesBuilder {
        <CorrectRootCauseRequestCauseLocationsItemLinesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CorrectRootCauseRequestCauseLocationsItemLinesBuilder {
    start: Option<i64>,
    end: Option<i64>,
}

impl CorrectRootCauseRequestCauseLocationsItemLinesBuilder {
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: i64) -> Self {
        self.end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CorrectRootCauseRequestCauseLocationsItemLines`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start`](CorrectRootCauseRequestCauseLocationsItemLinesBuilder::start)
    /// - [`end`](CorrectRootCauseRequestCauseLocationsItemLinesBuilder::end)
    pub fn build(self) -> Result<CorrectRootCauseRequestCauseLocationsItemLines, BuildError> {
        Ok(CorrectRootCauseRequestCauseLocationsItemLines {
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
        })
    }
}

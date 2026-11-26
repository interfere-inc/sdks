pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CorrectFixRequestLocationsItemLines {
    #[serde(default)]
    pub start: i64,
    #[serde(default)]
    pub end: i64,
}

impl CorrectFixRequestLocationsItemLines {
    pub fn builder() -> CorrectFixRequestLocationsItemLinesBuilder {
        <CorrectFixRequestLocationsItemLinesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CorrectFixRequestLocationsItemLinesBuilder {
    start: Option<i64>,
    end: Option<i64>,
}

impl CorrectFixRequestLocationsItemLinesBuilder {
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: i64) -> Self {
        self.end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CorrectFixRequestLocationsItemLines`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start`](CorrectFixRequestLocationsItemLinesBuilder::start)
    /// - [`end`](CorrectFixRequestLocationsItemLinesBuilder::end)
    pub fn build(self) -> Result<CorrectFixRequestLocationsItemLines, BuildError> {
        Ok(CorrectFixRequestLocationsItemLines {
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
        })
    }
}

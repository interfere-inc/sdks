pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AffectedByProblemTimeseriesResponseDataItem {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub day: NaiveDate,
}

impl AffectedByProblemTimeseriesResponseDataItem {
    pub fn builder() -> AffectedByProblemTimeseriesResponseDataItemBuilder {
        <AffectedByProblemTimeseriesResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AffectedByProblemTimeseriesResponseDataItemBuilder {
    count: Option<i64>,
    day: Option<NaiveDate>,
}

impl AffectedByProblemTimeseriesResponseDataItemBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn day(mut self, value: NaiveDate) -> Self {
        self.day = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AffectedByProblemTimeseriesResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](AffectedByProblemTimeseriesResponseDataItemBuilder::count)
    /// - [`day`](AffectedByProblemTimeseriesResponseDataItemBuilder::day)
    pub fn build(self) -> Result<AffectedByProblemTimeseriesResponseDataItem, BuildError> {
        Ok(AffectedByProblemTimeseriesResponseDataItem {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
        })
    }
}

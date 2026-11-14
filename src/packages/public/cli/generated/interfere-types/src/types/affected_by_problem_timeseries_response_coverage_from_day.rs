pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AffectedByProblemTimeseriesResponseCoverageFromDay {
    pub completeness: AffectedByProblemTimeseriesResponseCoverageFromDayCompleteness,
    pub counts: AffectedByProblemTimeseriesResponseCoverageFromDayCounts,
    pub scope: AffectedByProblemTimeseriesResponseCoverageFromDayScope,
    #[serde(rename = "fromDay")]
    #[serde(default)]
    pub from_day: NaiveDate,
    #[serde(rename = "toDayExclusive")]
    #[serde(default)]
    pub to_day_exclusive: NaiveDate,
    pub bucket: AffectedByProblemTimeseriesResponseCoverageFromDayBucket,
}

impl AffectedByProblemTimeseriesResponseCoverageFromDay {
    pub fn builder() -> AffectedByProblemTimeseriesResponseCoverageFromDayBuilder {
        <AffectedByProblemTimeseriesResponseCoverageFromDayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AffectedByProblemTimeseriesResponseCoverageFromDayBuilder {
    completeness: Option<AffectedByProblemTimeseriesResponseCoverageFromDayCompleteness>,
    counts: Option<AffectedByProblemTimeseriesResponseCoverageFromDayCounts>,
    scope: Option<AffectedByProblemTimeseriesResponseCoverageFromDayScope>,
    from_day: Option<NaiveDate>,
    to_day_exclusive: Option<NaiveDate>,
    bucket: Option<AffectedByProblemTimeseriesResponseCoverageFromDayBucket>,
}

impl AffectedByProblemTimeseriesResponseCoverageFromDayBuilder {
    pub fn completeness(mut self, value: AffectedByProblemTimeseriesResponseCoverageFromDayCompleteness) -> Self {
        self.completeness = Some(value);
        self
    }

    pub fn counts(mut self, value: AffectedByProblemTimeseriesResponseCoverageFromDayCounts) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn scope(mut self, value: AffectedByProblemTimeseriesResponseCoverageFromDayScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn from_day(mut self, value: NaiveDate) -> Self {
        self.from_day = Some(value);
        self
    }

    pub fn to_day_exclusive(mut self, value: NaiveDate) -> Self {
        self.to_day_exclusive = Some(value);
        self
    }

    pub fn bucket(mut self, value: AffectedByProblemTimeseriesResponseCoverageFromDayBucket) -> Self {
        self.bucket = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AffectedByProblemTimeseriesResponseCoverageFromDay`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completeness`](AffectedByProblemTimeseriesResponseCoverageFromDayBuilder::completeness)
    /// - [`counts`](AffectedByProblemTimeseriesResponseCoverageFromDayBuilder::counts)
    /// - [`scope`](AffectedByProblemTimeseriesResponseCoverageFromDayBuilder::scope)
    /// - [`from_day`](AffectedByProblemTimeseriesResponseCoverageFromDayBuilder::from_day)
    /// - [`to_day_exclusive`](AffectedByProblemTimeseriesResponseCoverageFromDayBuilder::to_day_exclusive)
    /// - [`bucket`](AffectedByProblemTimeseriesResponseCoverageFromDayBuilder::bucket)
    pub fn build(self) -> Result<AffectedByProblemTimeseriesResponseCoverageFromDay, BuildError> {
        Ok(AffectedByProblemTimeseriesResponseCoverageFromDay {
            completeness: self.completeness.ok_or_else(|| BuildError::missing_field("completeness"))?,
            counts: self.counts.ok_or_else(|| BuildError::missing_field("counts"))?,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            from_day: self.from_day.ok_or_else(|| BuildError::missing_field("from_day"))?,
            to_day_exclusive: self.to_day_exclusive.ok_or_else(|| BuildError::missing_field("to_day_exclusive"))?,
            bucket: self.bucket.ok_or_else(|| BuildError::missing_field("bucket"))?,
        })
    }
}

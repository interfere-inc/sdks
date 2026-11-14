pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemTimeseriesResponseCoverageFromDay {
    pub completeness: ProblemTimeseriesResponseCoverageFromDayCompleteness,
    pub counts: ProblemTimeseriesResponseCoverageFromDayCounts,
    pub scope: ProblemTimeseriesResponseCoverageFromDayScope,
    #[serde(rename = "fromDay")]
    #[serde(default)]
    pub from_day: NaiveDate,
    #[serde(rename = "toDayExclusive")]
    #[serde(default)]
    pub to_day_exclusive: NaiveDate,
    pub bucket: ProblemTimeseriesResponseCoverageFromDayBucket,
}

impl ProblemTimeseriesResponseCoverageFromDay {
    pub fn builder() -> ProblemTimeseriesResponseCoverageFromDayBuilder {
        <ProblemTimeseriesResponseCoverageFromDayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemTimeseriesResponseCoverageFromDayBuilder {
    completeness: Option<ProblemTimeseriesResponseCoverageFromDayCompleteness>,
    counts: Option<ProblemTimeseriesResponseCoverageFromDayCounts>,
    scope: Option<ProblemTimeseriesResponseCoverageFromDayScope>,
    from_day: Option<NaiveDate>,
    to_day_exclusive: Option<NaiveDate>,
    bucket: Option<ProblemTimeseriesResponseCoverageFromDayBucket>,
}

impl ProblemTimeseriesResponseCoverageFromDayBuilder {
    pub fn completeness(mut self, value: ProblemTimeseriesResponseCoverageFromDayCompleteness) -> Self {
        self.completeness = Some(value);
        self
    }

    pub fn counts(mut self, value: ProblemTimeseriesResponseCoverageFromDayCounts) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn scope(mut self, value: ProblemTimeseriesResponseCoverageFromDayScope) -> Self {
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

    pub fn bucket(mut self, value: ProblemTimeseriesResponseCoverageFromDayBucket) -> Self {
        self.bucket = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemTimeseriesResponseCoverageFromDay`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completeness`](ProblemTimeseriesResponseCoverageFromDayBuilder::completeness)
    /// - [`counts`](ProblemTimeseriesResponseCoverageFromDayBuilder::counts)
    /// - [`scope`](ProblemTimeseriesResponseCoverageFromDayBuilder::scope)
    /// - [`from_day`](ProblemTimeseriesResponseCoverageFromDayBuilder::from_day)
    /// - [`to_day_exclusive`](ProblemTimeseriesResponseCoverageFromDayBuilder::to_day_exclusive)
    /// - [`bucket`](ProblemTimeseriesResponseCoverageFromDayBuilder::bucket)
    pub fn build(self) -> Result<ProblemTimeseriesResponseCoverageFromDay, BuildError> {
        Ok(ProblemTimeseriesResponseCoverageFromDay {
            completeness: self.completeness.ok_or_else(|| BuildError::missing_field("completeness"))?,
            counts: self.counts.ok_or_else(|| BuildError::missing_field("counts"))?,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            from_day: self.from_day.ok_or_else(|| BuildError::missing_field("from_day"))?,
            to_day_exclusive: self.to_day_exclusive.ok_or_else(|| BuildError::missing_field("to_day_exclusive"))?,
            bucket: self.bucket.ok_or_else(|| BuildError::missing_field("bucket"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AffectedByProblemResponseCoverageFromDay {
    pub completeness: AffectedByProblemResponseCoverageFromDayCompleteness,
    pub counts: AffectedByProblemResponseCoverageFromDayCounts,
    pub scope: AffectedByProblemResponseCoverageFromDayScope,
    #[serde(rename = "fromDay")]
    #[serde(default)]
    pub from_day: NaiveDate,
    #[serde(rename = "toDayExclusive")]
    #[serde(default)]
    pub to_day_exclusive: NaiveDate,
    pub bucket: AffectedByProblemResponseCoverageFromDayBucket,
}

impl AffectedByProblemResponseCoverageFromDay {
    pub fn builder() -> AffectedByProblemResponseCoverageFromDayBuilder {
        <AffectedByProblemResponseCoverageFromDayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AffectedByProblemResponseCoverageFromDayBuilder {
    completeness: Option<AffectedByProblemResponseCoverageFromDayCompleteness>,
    counts: Option<AffectedByProblemResponseCoverageFromDayCounts>,
    scope: Option<AffectedByProblemResponseCoverageFromDayScope>,
    from_day: Option<NaiveDate>,
    to_day_exclusive: Option<NaiveDate>,
    bucket: Option<AffectedByProblemResponseCoverageFromDayBucket>,
}

impl AffectedByProblemResponseCoverageFromDayBuilder {
    pub fn completeness(mut self, value: AffectedByProblemResponseCoverageFromDayCompleteness) -> Self {
        self.completeness = Some(value);
        self
    }

    pub fn counts(mut self, value: AffectedByProblemResponseCoverageFromDayCounts) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn scope(mut self, value: AffectedByProblemResponseCoverageFromDayScope) -> Self {
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

    pub fn bucket(mut self, value: AffectedByProblemResponseCoverageFromDayBucket) -> Self {
        self.bucket = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AffectedByProblemResponseCoverageFromDay`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completeness`](AffectedByProblemResponseCoverageFromDayBuilder::completeness)
    /// - [`counts`](AffectedByProblemResponseCoverageFromDayBuilder::counts)
    /// - [`scope`](AffectedByProblemResponseCoverageFromDayBuilder::scope)
    /// - [`from_day`](AffectedByProblemResponseCoverageFromDayBuilder::from_day)
    /// - [`to_day_exclusive`](AffectedByProblemResponseCoverageFromDayBuilder::to_day_exclusive)
    /// - [`bucket`](AffectedByProblemResponseCoverageFromDayBuilder::bucket)
    pub fn build(self) -> Result<AffectedByProblemResponseCoverageFromDay, BuildError> {
        Ok(AffectedByProblemResponseCoverageFromDay {
            completeness: self.completeness.ok_or_else(|| BuildError::missing_field("completeness"))?,
            counts: self.counts.ok_or_else(|| BuildError::missing_field("counts"))?,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            from_day: self.from_day.ok_or_else(|| BuildError::missing_field("from_day"))?,
            to_day_exclusive: self.to_day_exclusive.ok_or_else(|| BuildError::missing_field("to_day_exclusive"))?,
            bucket: self.bucket.ok_or_else(|| BuildError::missing_field("bucket"))?,
        })
    }
}

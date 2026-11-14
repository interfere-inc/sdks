pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemTimeseriesResponseCoverageZero {
    pub completeness: ProblemTimeseriesResponseCoverageZeroCompleteness,
    pub counts: ProblemTimeseriesResponseCoverageZeroCounts,
    pub scope: ProblemTimeseriesResponseCoverageZeroScope,
    pub bucket: ProblemTimeseriesResponseCoverageZeroBucket,
}

impl ProblemTimeseriesResponseCoverageZero {
    pub fn builder() -> ProblemTimeseriesResponseCoverageZeroBuilder {
        <ProblemTimeseriesResponseCoverageZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemTimeseriesResponseCoverageZeroBuilder {
    completeness: Option<ProblemTimeseriesResponseCoverageZeroCompleteness>,
    counts: Option<ProblemTimeseriesResponseCoverageZeroCounts>,
    scope: Option<ProblemTimeseriesResponseCoverageZeroScope>,
    bucket: Option<ProblemTimeseriesResponseCoverageZeroBucket>,
}

impl ProblemTimeseriesResponseCoverageZeroBuilder {
    pub fn completeness(mut self, value: ProblemTimeseriesResponseCoverageZeroCompleteness) -> Self {
        self.completeness = Some(value);
        self
    }

    pub fn counts(mut self, value: ProblemTimeseriesResponseCoverageZeroCounts) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn scope(mut self, value: ProblemTimeseriesResponseCoverageZeroScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn bucket(mut self, value: ProblemTimeseriesResponseCoverageZeroBucket) -> Self {
        self.bucket = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemTimeseriesResponseCoverageZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completeness`](ProblemTimeseriesResponseCoverageZeroBuilder::completeness)
    /// - [`counts`](ProblemTimeseriesResponseCoverageZeroBuilder::counts)
    /// - [`scope`](ProblemTimeseriesResponseCoverageZeroBuilder::scope)
    /// - [`bucket`](ProblemTimeseriesResponseCoverageZeroBuilder::bucket)
    pub fn build(self) -> Result<ProblemTimeseriesResponseCoverageZero, BuildError> {
        Ok(ProblemTimeseriesResponseCoverageZero {
            completeness: self.completeness.ok_or_else(|| BuildError::missing_field("completeness"))?,
            counts: self.counts.ok_or_else(|| BuildError::missing_field("counts"))?,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            bucket: self.bucket.ok_or_else(|| BuildError::missing_field("bucket"))?,
        })
    }
}

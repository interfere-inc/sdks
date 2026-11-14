pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AffectedByProblemTimeseriesResponseCoverageZero {
    pub completeness: AffectedByProblemTimeseriesResponseCoverageZeroCompleteness,
    pub counts: AffectedByProblemTimeseriesResponseCoverageZeroCounts,
    pub scope: AffectedByProblemTimeseriesResponseCoverageZeroScope,
    pub bucket: AffectedByProblemTimeseriesResponseCoverageZeroBucket,
}

impl AffectedByProblemTimeseriesResponseCoverageZero {
    pub fn builder() -> AffectedByProblemTimeseriesResponseCoverageZeroBuilder {
        <AffectedByProblemTimeseriesResponseCoverageZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AffectedByProblemTimeseriesResponseCoverageZeroBuilder {
    completeness: Option<AffectedByProblemTimeseriesResponseCoverageZeroCompleteness>,
    counts: Option<AffectedByProblemTimeseriesResponseCoverageZeroCounts>,
    scope: Option<AffectedByProblemTimeseriesResponseCoverageZeroScope>,
    bucket: Option<AffectedByProblemTimeseriesResponseCoverageZeroBucket>,
}

impl AffectedByProblemTimeseriesResponseCoverageZeroBuilder {
    pub fn completeness(mut self, value: AffectedByProblemTimeseriesResponseCoverageZeroCompleteness) -> Self {
        self.completeness = Some(value);
        self
    }

    pub fn counts(mut self, value: AffectedByProblemTimeseriesResponseCoverageZeroCounts) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn scope(mut self, value: AffectedByProblemTimeseriesResponseCoverageZeroScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn bucket(mut self, value: AffectedByProblemTimeseriesResponseCoverageZeroBucket) -> Self {
        self.bucket = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AffectedByProblemTimeseriesResponseCoverageZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completeness`](AffectedByProblemTimeseriesResponseCoverageZeroBuilder::completeness)
    /// - [`counts`](AffectedByProblemTimeseriesResponseCoverageZeroBuilder::counts)
    /// - [`scope`](AffectedByProblemTimeseriesResponseCoverageZeroBuilder::scope)
    /// - [`bucket`](AffectedByProblemTimeseriesResponseCoverageZeroBuilder::bucket)
    pub fn build(self) -> Result<AffectedByProblemTimeseriesResponseCoverageZero, BuildError> {
        Ok(AffectedByProblemTimeseriesResponseCoverageZero {
            completeness: self.completeness.ok_or_else(|| BuildError::missing_field("completeness"))?,
            counts: self.counts.ok_or_else(|| BuildError::missing_field("counts"))?,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            bucket: self.bucket.ok_or_else(|| BuildError::missing_field("bucket"))?,
        })
    }
}

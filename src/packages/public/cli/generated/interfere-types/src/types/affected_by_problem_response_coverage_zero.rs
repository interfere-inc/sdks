pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AffectedByProblemResponseCoverageZero {
    pub completeness: AffectedByProblemResponseCoverageZeroCompleteness,
    pub counts: AffectedByProblemResponseCoverageZeroCounts,
    pub scope: AffectedByProblemResponseCoverageZeroScope,
    pub bucket: AffectedByProblemResponseCoverageZeroBucket,
}

impl AffectedByProblemResponseCoverageZero {
    pub fn builder() -> AffectedByProblemResponseCoverageZeroBuilder {
        <AffectedByProblemResponseCoverageZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AffectedByProblemResponseCoverageZeroBuilder {
    completeness: Option<AffectedByProblemResponseCoverageZeroCompleteness>,
    counts: Option<AffectedByProblemResponseCoverageZeroCounts>,
    scope: Option<AffectedByProblemResponseCoverageZeroScope>,
    bucket: Option<AffectedByProblemResponseCoverageZeroBucket>,
}

impl AffectedByProblemResponseCoverageZeroBuilder {
    pub fn completeness(mut self, value: AffectedByProblemResponseCoverageZeroCompleteness) -> Self {
        self.completeness = Some(value);
        self
    }

    pub fn counts(mut self, value: AffectedByProblemResponseCoverageZeroCounts) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn scope(mut self, value: AffectedByProblemResponseCoverageZeroScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn bucket(mut self, value: AffectedByProblemResponseCoverageZeroBucket) -> Self {
        self.bucket = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AffectedByProblemResponseCoverageZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`completeness`](AffectedByProblemResponseCoverageZeroBuilder::completeness)
    /// - [`counts`](AffectedByProblemResponseCoverageZeroBuilder::counts)
    /// - [`scope`](AffectedByProblemResponseCoverageZeroBuilder::scope)
    /// - [`bucket`](AffectedByProblemResponseCoverageZeroBuilder::bucket)
    pub fn build(self) -> Result<AffectedByProblemResponseCoverageZero, BuildError> {
        Ok(AffectedByProblemResponseCoverageZero {
            completeness: self.completeness.ok_or_else(|| BuildError::missing_field("completeness"))?,
            counts: self.counts.ok_or_else(|| BuildError::missing_field("counts"))?,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            bucket: self.bucket.ok_or_else(|| BuildError::missing_field("bucket"))?,
        })
    }
}

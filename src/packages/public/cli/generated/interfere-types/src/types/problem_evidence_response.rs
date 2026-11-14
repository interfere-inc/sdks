pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProblemEvidenceResponse {
    /// Every live finding grouped into this Problem, newest first
    #[serde(default)]
    pub findings: Vec<ProblemEvidenceResponseFindingsItem>,
    /// Representative captured occurrences from the last 7 days with their exception messages; factId decodes to the trace and span to read with the trace endpoints
    #[serde(default)]
    pub facts: Vec<ProblemEvidenceResponseFactsItem>,
    /// The investigator's Observations in order, each with the evidence it cited (source files and lines, Facts, spans, logs, builds)
    #[serde(default)]
    pub observations: Vec<ProblemEvidenceResponseObservationsItem>,
}

impl ProblemEvidenceResponse {
    pub fn builder() -> ProblemEvidenceResponseBuilder {
        <ProblemEvidenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseBuilder {
    findings: Option<Vec<ProblemEvidenceResponseFindingsItem>>,
    facts: Option<Vec<ProblemEvidenceResponseFactsItem>>,
    observations: Option<Vec<ProblemEvidenceResponseObservationsItem>>,
}

impl ProblemEvidenceResponseBuilder {
    pub fn findings(mut self, value: Vec<ProblemEvidenceResponseFindingsItem>) -> Self {
        self.findings = Some(value);
        self
    }

    pub fn facts(mut self, value: Vec<ProblemEvidenceResponseFactsItem>) -> Self {
        self.facts = Some(value);
        self
    }

    pub fn observations(mut self, value: Vec<ProblemEvidenceResponseObservationsItem>) -> Self {
        self.observations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`findings`](ProblemEvidenceResponseBuilder::findings)
    /// - [`facts`](ProblemEvidenceResponseBuilder::facts)
    /// - [`observations`](ProblemEvidenceResponseBuilder::observations)
    pub fn build(self) -> Result<ProblemEvidenceResponse, BuildError> {
        Ok(ProblemEvidenceResponse {
            findings: self.findings.ok_or_else(|| BuildError::missing_field("findings"))?,
            facts: self.facts.ok_or_else(|| BuildError::missing_field("facts"))?,
            observations: self.observations.ok_or_else(|| BuildError::missing_field("observations"))?,
        })
    }
}

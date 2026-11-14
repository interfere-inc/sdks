pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArguments {
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArgumentsKind,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub arguments: HashMap<String, Option<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArgumentsArgumentsValue>>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArguments {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArgumentsBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArgumentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArgumentsBuilder {
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArgumentsKind>,
    name: Option<String>,
    arguments: Option<HashMap<String, Option<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArgumentsArgumentsValue>>>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArgumentsBuilder {
    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArgumentsKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn arguments(mut self, value: HashMap<String, Option<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArgumentsArgumentsValue>>) -> Self {
        self.arguments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArguments`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArgumentsBuilder::kind)
    /// - [`name`](ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArgumentsBuilder::name)
    /// - [`arguments`](ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArgumentsBuilder::arguments)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArguments, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationArguments {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            arguments: self.arguments.ok_or_else(|| BuildError::missing_field("arguments"))?,
        })
    }
}

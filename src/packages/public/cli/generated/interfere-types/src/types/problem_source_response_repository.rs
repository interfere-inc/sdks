pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProblemSourceResponseRepository {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "workingDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

impl ProblemSourceResponseRepository {
    pub fn builder() -> ProblemSourceResponseRepositoryBuilder {
        <ProblemSourceResponseRepositoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemSourceResponseRepositoryBuilder {
    name: Option<String>,
    working_directory: Option<String>,
}

impl ProblemSourceResponseRepositoryBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn working_directory(mut self, value: impl Into<String>) -> Self {
        self.working_directory = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemSourceResponseRepository`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ProblemSourceResponseRepositoryBuilder::name)
    pub fn build(self) -> Result<ProblemSourceResponseRepository, BuildError> {
        Ok(ProblemSourceResponseRepository {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            working_directory: self.working_directory,
        })
    }
}

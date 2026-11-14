pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KanbanProblemsRequestArgs {
    #[serde(default)]
    pub environments: Vec<KanbanProblemsRequestArgsEnvironmentsItem>,
}

impl KanbanProblemsRequestArgs {
    pub fn builder() -> KanbanProblemsRequestArgsBuilder {
        <KanbanProblemsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KanbanProblemsRequestArgsBuilder {
    environments: Option<Vec<KanbanProblemsRequestArgsEnvironmentsItem>>,
}

impl KanbanProblemsRequestArgsBuilder {
    pub fn environments(mut self, value: Vec<KanbanProblemsRequestArgsEnvironmentsItem>) -> Self {
        self.environments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KanbanProblemsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environments`](KanbanProblemsRequestArgsBuilder::environments)
    pub fn build(self) -> Result<KanbanProblemsRequestArgs, BuildError> {
        Ok(KanbanProblemsRequestArgs {
            environments: self.environments.ok_or_else(|| BuildError::missing_field("environments"))?,
        })
    }
}

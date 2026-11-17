pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KanbanProblemsRequestArgs {
    /// Workspace environment names from environments.list. A problem matches when any of its findings was reported in one of them.
    #[serde(rename = "environmentNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_names: Option<Vec<String>>,
    /// Deprecated: use environmentNames. App (surface) environment identifiers, matched only on findings from that app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<KanbanProblemsRequestArgsEnvironmentsItem>>,
}

impl KanbanProblemsRequestArgs {
    pub fn builder() -> KanbanProblemsRequestArgsBuilder {
        <KanbanProblemsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KanbanProblemsRequestArgsBuilder {
    environment_names: Option<Vec<String>>,
    environments: Option<Vec<KanbanProblemsRequestArgsEnvironmentsItem>>,
}

impl KanbanProblemsRequestArgsBuilder {
    pub fn environment_names(mut self, value: Vec<String>) -> Self {
        self.environment_names = Some(value);
        self
    }

    pub fn environments(mut self, value: Vec<KanbanProblemsRequestArgsEnvironmentsItem>) -> Self {
        self.environments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KanbanProblemsRequestArgs`].
    pub fn build(self) -> Result<KanbanProblemsRequestArgs, BuildError> {
        Ok(KanbanProblemsRequestArgs {
            environment_names: self.environment_names,
            environments: self.environments,
        })
    }
}

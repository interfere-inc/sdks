pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KanbanProblemsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<KanbanProblemsRequestArgs>,
}

impl KanbanProblemsRequest {
    pub fn builder() -> KanbanProblemsRequestBuilder {
        <KanbanProblemsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KanbanProblemsRequestBuilder {
    args: Option<KanbanProblemsRequestArgs>,
}

impl KanbanProblemsRequestBuilder {
    pub fn args(mut self, value: KanbanProblemsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KanbanProblemsRequest`].
    pub fn build(self) -> Result<KanbanProblemsRequest, BuildError> {
        Ok(KanbanProblemsRequest {
            args: self.args,
        })
    }
}


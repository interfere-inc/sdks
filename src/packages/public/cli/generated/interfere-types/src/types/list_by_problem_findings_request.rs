pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListByProblemFindingsRequest {
    #[serde(default)]
    pub args: ListByProblemFindingsRequestArgs,
}

impl ListByProblemFindingsRequest {
    pub fn builder() -> ListByProblemFindingsRequestBuilder {
        <ListByProblemFindingsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListByProblemFindingsRequestBuilder {
    args: Option<ListByProblemFindingsRequestArgs>,
}

impl ListByProblemFindingsRequestBuilder {
    pub fn args(mut self, value: ListByProblemFindingsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListByProblemFindingsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ListByProblemFindingsRequestBuilder::args)
    pub fn build(self) -> Result<ListByProblemFindingsRequest, BuildError> {
        Ok(ListByProblemFindingsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


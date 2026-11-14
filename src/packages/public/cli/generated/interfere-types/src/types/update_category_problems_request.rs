pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCategoryProblemsRequest {
    #[serde(default)]
    pub args: UpdateCategoryProblemsRequestArgs,
}

impl UpdateCategoryProblemsRequest {
    pub fn builder() -> UpdateCategoryProblemsRequestBuilder {
        <UpdateCategoryProblemsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCategoryProblemsRequestBuilder {
    args: Option<UpdateCategoryProblemsRequestArgs>,
}

impl UpdateCategoryProblemsRequestBuilder {
    pub fn args(mut self, value: UpdateCategoryProblemsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCategoryProblemsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateCategoryProblemsRequestBuilder::args)
    pub fn build(self) -> Result<UpdateCategoryProblemsRequest, BuildError> {
        Ok(UpdateCategoryProblemsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


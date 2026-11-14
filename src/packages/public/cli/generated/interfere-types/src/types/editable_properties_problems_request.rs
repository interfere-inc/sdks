pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EditablePropertiesProblemsRequest {
    #[serde(default)]
    pub args: EditablePropertiesProblemsRequestArgs,
}

impl EditablePropertiesProblemsRequest {
    pub fn builder() -> EditablePropertiesProblemsRequestBuilder {
        <EditablePropertiesProblemsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditablePropertiesProblemsRequestBuilder {
    args: Option<EditablePropertiesProblemsRequestArgs>,
}

impl EditablePropertiesProblemsRequestBuilder {
    pub fn args(mut self, value: EditablePropertiesProblemsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EditablePropertiesProblemsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](EditablePropertiesProblemsRequestBuilder::args)
    pub fn build(self) -> Result<EditablePropertiesProblemsRequest, BuildError> {
        Ok(EditablePropertiesProblemsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetConsumersRequestArgs {
    #[serde(rename = "endUserId")]
    #[serde(default)]
    pub end_user_id: String,
}

impl GetConsumersRequestArgs {
    pub fn builder() -> GetConsumersRequestArgsBuilder {
        <GetConsumersRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetConsumersRequestArgsBuilder {
    end_user_id: Option<String>,
}

impl GetConsumersRequestArgsBuilder {
    pub fn end_user_id(mut self, value: impl Into<String>) -> Self {
        self.end_user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetConsumersRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end_user_id`](GetConsumersRequestArgsBuilder::end_user_id)
    pub fn build(self) -> Result<GetConsumersRequestArgs, BuildError> {
        Ok(GetConsumersRequestArgs {
            end_user_id: self.end_user_id.ok_or_else(|| BuildError::missing_field("end_user_id"))?,
        })
    }
}

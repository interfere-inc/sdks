pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSupportThreadResponse {
    #[serde(default)]
    pub id: String,
}

impl CreateSupportThreadResponse {
    pub fn builder() -> CreateSupportThreadResponseBuilder {
        <CreateSupportThreadResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSupportThreadResponseBuilder {
    id: Option<String>,
}

impl CreateSupportThreadResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSupportThreadResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateSupportThreadResponseBuilder::id)
    pub fn build(self) -> Result<CreateSupportThreadResponse, BuildError> {
        Ok(CreateSupportThreadResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

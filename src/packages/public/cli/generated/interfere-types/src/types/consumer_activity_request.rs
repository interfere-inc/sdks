pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConsumerActivityRequest {
    #[serde(rename = "endUserIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_user_ids: Option<Vec<String>>,
}

impl ConsumerActivityRequest {
    pub fn builder() -> ConsumerActivityRequestBuilder {
        <ConsumerActivityRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerActivityRequestBuilder {
    end_user_ids: Option<Vec<String>>,
}

impl ConsumerActivityRequestBuilder {
    pub fn end_user_ids(mut self, value: Vec<String>) -> Self {
        self.end_user_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsumerActivityRequest`].
    pub fn build(self) -> Result<ConsumerActivityRequest, BuildError> {
        Ok(ConsumerActivityRequest {
            end_user_ids: self.end_user_ids,
        })
    }
}


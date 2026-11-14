pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for consumerLookup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConsumerLookupQueryRequest {
    /// Email address of the application user, compared case-insensitively. An exact match on the identifier your application sends with identify() also counts.
    #[serde(default)]
    pub email: String,
}

impl ConsumerLookupQueryRequest {
    pub fn builder() -> ConsumerLookupQueryRequestBuilder {
        <ConsumerLookupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerLookupQueryRequestBuilder {
    email: Option<String>,
}

impl ConsumerLookupQueryRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConsumerLookupQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](ConsumerLookupQueryRequestBuilder::email)
    pub fn build(self) -> Result<ConsumerLookupQueryRequest, BuildError> {
        Ok(ConsumerLookupQueryRequest {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
        })
    }
}


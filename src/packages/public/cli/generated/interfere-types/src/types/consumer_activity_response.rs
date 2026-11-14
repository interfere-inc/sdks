pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConsumerActivityResponse {
    #[serde(default)]
    pub users: HashMap<String, ConsumerActivityResponseUsersValue>,
}

impl ConsumerActivityResponse {
    pub fn builder() -> ConsumerActivityResponseBuilder {
        <ConsumerActivityResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerActivityResponseBuilder {
    users: Option<HashMap<String, ConsumerActivityResponseUsersValue>>,
}

impl ConsumerActivityResponseBuilder {
    pub fn users(mut self, value: HashMap<String, ConsumerActivityResponseUsersValue>) -> Self {
        self.users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsumerActivityResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`users`](ConsumerActivityResponseBuilder::users)
    pub fn build(self) -> Result<ConsumerActivityResponse, BuildError> {
        Ok(ConsumerActivityResponse {
            users: self.users.ok_or_else(|| BuildError::missing_field("users"))?,
        })
    }
}

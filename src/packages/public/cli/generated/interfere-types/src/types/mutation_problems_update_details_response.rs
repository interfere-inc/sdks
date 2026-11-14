pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationProblemsUpdateDetailsResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationProblemsUpdateDetailsResponse {
    pub fn builder() -> MutationProblemsUpdateDetailsResponseBuilder {
        <MutationProblemsUpdateDetailsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationProblemsUpdateDetailsResponseBuilder {
    success: Option<bool>,
}

impl MutationProblemsUpdateDetailsResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationProblemsUpdateDetailsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationProblemsUpdateDetailsResponseBuilder::success)
    pub fn build(self) -> Result<MutationProblemsUpdateDetailsResponse, BuildError> {
        Ok(MutationProblemsUpdateDetailsResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

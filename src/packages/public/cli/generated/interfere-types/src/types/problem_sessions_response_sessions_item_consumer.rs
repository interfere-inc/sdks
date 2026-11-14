pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProblemSessionsResponseSessionsItemConsumer {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

impl ProblemSessionsResponseSessionsItemConsumer {
    pub fn builder() -> ProblemSessionsResponseSessionsItemConsumerBuilder {
        <ProblemSessionsResponseSessionsItemConsumerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemSessionsResponseSessionsItemConsumerBuilder {
    id: Option<String>,
    identifier: Option<String>,
    name: Option<String>,
    email: Option<String>,
    avatar: Option<String>,
}

impl ProblemSessionsResponseSessionsItemConsumerBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn avatar(mut self, value: impl Into<String>) -> Self {
        self.avatar = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemSessionsResponseSessionsItemConsumer`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProblemSessionsResponseSessionsItemConsumerBuilder::id)
    /// - [`identifier`](ProblemSessionsResponseSessionsItemConsumerBuilder::identifier)
    pub fn build(self) -> Result<ProblemSessionsResponseSessionsItemConsumer, BuildError> {
        Ok(ProblemSessionsResponseSessionsItemConsumer {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            identifier: self.identifier.ok_or_else(|| BuildError::missing_field("identifier"))?,
            name: self.name,
            email: self.email,
            avatar: self.avatar,
        })
    }
}

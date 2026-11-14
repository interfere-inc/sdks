pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConsumerDetailsResponseSourceOne {
    pub name: ConsumerDetailsResponseSourceOneName,
    pub r#type: ConsumerDetailsResponseSourceOneType,
}

impl ConsumerDetailsResponseSourceOne {
    pub fn builder() -> ConsumerDetailsResponseSourceOneBuilder {
        <ConsumerDetailsResponseSourceOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerDetailsResponseSourceOneBuilder {
    name: Option<ConsumerDetailsResponseSourceOneName>,
    r#type: Option<ConsumerDetailsResponseSourceOneType>,
}

impl ConsumerDetailsResponseSourceOneBuilder {
    pub fn name(mut self, value: ConsumerDetailsResponseSourceOneName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: ConsumerDetailsResponseSourceOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsumerDetailsResponseSourceOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConsumerDetailsResponseSourceOneBuilder::name)
    /// - [`r#type`](ConsumerDetailsResponseSourceOneBuilder::r#type)
    pub fn build(self) -> Result<ConsumerDetailsResponseSourceOne, BuildError> {
        Ok(ConsumerDetailsResponseSourceOne {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

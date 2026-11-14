pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConsumerDetailsResponseSourceZero {
    pub name: ConsumerDetailsResponseSourceZeroName,
    pub r#type: ConsumerDetailsResponseSourceZeroType,
}

impl ConsumerDetailsResponseSourceZero {
    pub fn builder() -> ConsumerDetailsResponseSourceZeroBuilder {
        <ConsumerDetailsResponseSourceZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerDetailsResponseSourceZeroBuilder {
    name: Option<ConsumerDetailsResponseSourceZeroName>,
    r#type: Option<ConsumerDetailsResponseSourceZeroType>,
}

impl ConsumerDetailsResponseSourceZeroBuilder {
    pub fn name(mut self, value: ConsumerDetailsResponseSourceZeroName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: ConsumerDetailsResponseSourceZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsumerDetailsResponseSourceZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConsumerDetailsResponseSourceZeroBuilder::name)
    /// - [`r#type`](ConsumerDetailsResponseSourceZeroBuilder::r#type)
    pub fn build(self) -> Result<ConsumerDetailsResponseSourceZero, BuildError> {
        Ok(ConsumerDetailsResponseSourceZero {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

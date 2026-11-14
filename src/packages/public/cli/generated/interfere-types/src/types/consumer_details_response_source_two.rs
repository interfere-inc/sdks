pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConsumerDetailsResponseSourceTwo {
    pub name: ConsumerDetailsResponseSourceTwoName,
    pub r#type: ConsumerDetailsResponseSourceTwoType,
}

impl ConsumerDetailsResponseSourceTwo {
    pub fn builder() -> ConsumerDetailsResponseSourceTwoBuilder {
        <ConsumerDetailsResponseSourceTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerDetailsResponseSourceTwoBuilder {
    name: Option<ConsumerDetailsResponseSourceTwoName>,
    r#type: Option<ConsumerDetailsResponseSourceTwoType>,
}

impl ConsumerDetailsResponseSourceTwoBuilder {
    pub fn name(mut self, value: ConsumerDetailsResponseSourceTwoName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: ConsumerDetailsResponseSourceTwoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsumerDetailsResponseSourceTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConsumerDetailsResponseSourceTwoBuilder::name)
    /// - [`r#type`](ConsumerDetailsResponseSourceTwoBuilder::r#type)
    pub fn build(self) -> Result<ConsumerDetailsResponseSourceTwo, BuildError> {
        Ok(ConsumerDetailsResponseSourceTwo {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConsumerDetailsResponseSourceThree {
    #[serde(default)]
    pub name: String,
    pub r#type: ConsumerDetailsResponseSourceThreeType,
}

impl ConsumerDetailsResponseSourceThree {
    pub fn builder() -> ConsumerDetailsResponseSourceThreeBuilder {
        <ConsumerDetailsResponseSourceThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerDetailsResponseSourceThreeBuilder {
    name: Option<String>,
    r#type: Option<ConsumerDetailsResponseSourceThreeType>,
}

impl ConsumerDetailsResponseSourceThreeBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ConsumerDetailsResponseSourceThreeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsumerDetailsResponseSourceThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConsumerDetailsResponseSourceThreeBuilder::name)
    /// - [`r#type`](ConsumerDetailsResponseSourceThreeBuilder::r#type)
    pub fn build(self) -> Result<ConsumerDetailsResponseSourceThree, BuildError> {
        Ok(ConsumerDetailsResponseSourceThree {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

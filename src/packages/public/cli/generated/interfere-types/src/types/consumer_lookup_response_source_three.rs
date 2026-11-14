pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConsumerLookupResponseSourceThree {
    #[serde(default)]
    pub name: String,
    pub r#type: ConsumerLookupResponseSourceThreeType,
}

impl ConsumerLookupResponseSourceThree {
    pub fn builder() -> ConsumerLookupResponseSourceThreeBuilder {
        <ConsumerLookupResponseSourceThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerLookupResponseSourceThreeBuilder {
    name: Option<String>,
    r#type: Option<ConsumerLookupResponseSourceThreeType>,
}

impl ConsumerLookupResponseSourceThreeBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ConsumerLookupResponseSourceThreeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsumerLookupResponseSourceThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConsumerLookupResponseSourceThreeBuilder::name)
    /// - [`r#type`](ConsumerLookupResponseSourceThreeBuilder::r#type)
    pub fn build(self) -> Result<ConsumerLookupResponseSourceThree, BuildError> {
        Ok(ConsumerLookupResponseSourceThree {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

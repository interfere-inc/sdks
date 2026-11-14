pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConsumerLookupResponseSourceOne {
    pub name: ConsumerLookupResponseSourceOneName,
    pub r#type: ConsumerLookupResponseSourceOneType,
}

impl ConsumerLookupResponseSourceOne {
    pub fn builder() -> ConsumerLookupResponseSourceOneBuilder {
        <ConsumerLookupResponseSourceOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerLookupResponseSourceOneBuilder {
    name: Option<ConsumerLookupResponseSourceOneName>,
    r#type: Option<ConsumerLookupResponseSourceOneType>,
}

impl ConsumerLookupResponseSourceOneBuilder {
    pub fn name(mut self, value: ConsumerLookupResponseSourceOneName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: ConsumerLookupResponseSourceOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsumerLookupResponseSourceOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConsumerLookupResponseSourceOneBuilder::name)
    /// - [`r#type`](ConsumerLookupResponseSourceOneBuilder::r#type)
    pub fn build(self) -> Result<ConsumerLookupResponseSourceOne, BuildError> {
        Ok(ConsumerLookupResponseSourceOne {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

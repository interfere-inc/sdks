pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConsumerLookupResponseSourceZero {
    pub name: ConsumerLookupResponseSourceZeroName,
    pub r#type: ConsumerLookupResponseSourceZeroType,
}

impl ConsumerLookupResponseSourceZero {
    pub fn builder() -> ConsumerLookupResponseSourceZeroBuilder {
        <ConsumerLookupResponseSourceZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerLookupResponseSourceZeroBuilder {
    name: Option<ConsumerLookupResponseSourceZeroName>,
    r#type: Option<ConsumerLookupResponseSourceZeroType>,
}

impl ConsumerLookupResponseSourceZeroBuilder {
    pub fn name(mut self, value: ConsumerLookupResponseSourceZeroName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: ConsumerLookupResponseSourceZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsumerLookupResponseSourceZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConsumerLookupResponseSourceZeroBuilder::name)
    /// - [`r#type`](ConsumerLookupResponseSourceZeroBuilder::r#type)
    pub fn build(self) -> Result<ConsumerLookupResponseSourceZero, BuildError> {
        Ok(ConsumerLookupResponseSourceZero {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

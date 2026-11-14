pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConsumerLookupResponseSourceTwo {
    pub name: ConsumerLookupResponseSourceTwoName,
    pub r#type: ConsumerLookupResponseSourceTwoType,
}

impl ConsumerLookupResponseSourceTwo {
    pub fn builder() -> ConsumerLookupResponseSourceTwoBuilder {
        <ConsumerLookupResponseSourceTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerLookupResponseSourceTwoBuilder {
    name: Option<ConsumerLookupResponseSourceTwoName>,
    r#type: Option<ConsumerLookupResponseSourceTwoType>,
}

impl ConsumerLookupResponseSourceTwoBuilder {
    pub fn name(mut self, value: ConsumerLookupResponseSourceTwoName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: ConsumerLookupResponseSourceTwoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsumerLookupResponseSourceTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConsumerLookupResponseSourceTwoBuilder::name)
    /// - [`r#type`](ConsumerLookupResponseSourceTwoBuilder::r#type)
    pub fn build(self) -> Result<ConsumerLookupResponseSourceTwo, BuildError> {
        Ok(ConsumerLookupResponseSourceTwo {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryConsumersGetByIdResponseSourceTwo {
    pub name: QueryConsumersGetByIdResponseSourceTwoName,
    pub r#type: QueryConsumersGetByIdResponseSourceTwoType,
}

impl QueryConsumersGetByIdResponseSourceTwo {
    pub fn builder() -> QueryConsumersGetByIdResponseSourceTwoBuilder {
        <QueryConsumersGetByIdResponseSourceTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryConsumersGetByIdResponseSourceTwoBuilder {
    name: Option<QueryConsumersGetByIdResponseSourceTwoName>,
    r#type: Option<QueryConsumersGetByIdResponseSourceTwoType>,
}

impl QueryConsumersGetByIdResponseSourceTwoBuilder {
    pub fn name(mut self, value: QueryConsumersGetByIdResponseSourceTwoName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryConsumersGetByIdResponseSourceTwoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryConsumersGetByIdResponseSourceTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryConsumersGetByIdResponseSourceTwoBuilder::name)
    /// - [`r#type`](QueryConsumersGetByIdResponseSourceTwoBuilder::r#type)
    pub fn build(self) -> Result<QueryConsumersGetByIdResponseSourceTwo, BuildError> {
        Ok(QueryConsumersGetByIdResponseSourceTwo {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

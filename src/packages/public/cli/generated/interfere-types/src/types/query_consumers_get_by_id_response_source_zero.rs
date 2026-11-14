pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryConsumersGetByIdResponseSourceZero {
    pub name: QueryConsumersGetByIdResponseSourceZeroName,
    pub r#type: QueryConsumersGetByIdResponseSourceZeroType,
}

impl QueryConsumersGetByIdResponseSourceZero {
    pub fn builder() -> QueryConsumersGetByIdResponseSourceZeroBuilder {
        <QueryConsumersGetByIdResponseSourceZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryConsumersGetByIdResponseSourceZeroBuilder {
    name: Option<QueryConsumersGetByIdResponseSourceZeroName>,
    r#type: Option<QueryConsumersGetByIdResponseSourceZeroType>,
}

impl QueryConsumersGetByIdResponseSourceZeroBuilder {
    pub fn name(mut self, value: QueryConsumersGetByIdResponseSourceZeroName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryConsumersGetByIdResponseSourceZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryConsumersGetByIdResponseSourceZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryConsumersGetByIdResponseSourceZeroBuilder::name)
    /// - [`r#type`](QueryConsumersGetByIdResponseSourceZeroBuilder::r#type)
    pub fn build(self) -> Result<QueryConsumersGetByIdResponseSourceZero, BuildError> {
        Ok(QueryConsumersGetByIdResponseSourceZero {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

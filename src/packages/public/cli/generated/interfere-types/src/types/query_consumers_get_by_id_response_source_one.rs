pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryConsumersGetByIdResponseSourceOne {
    pub name: QueryConsumersGetByIdResponseSourceOneName,
    pub r#type: QueryConsumersGetByIdResponseSourceOneType,
}

impl QueryConsumersGetByIdResponseSourceOne {
    pub fn builder() -> QueryConsumersGetByIdResponseSourceOneBuilder {
        <QueryConsumersGetByIdResponseSourceOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryConsumersGetByIdResponseSourceOneBuilder {
    name: Option<QueryConsumersGetByIdResponseSourceOneName>,
    r#type: Option<QueryConsumersGetByIdResponseSourceOneType>,
}

impl QueryConsumersGetByIdResponseSourceOneBuilder {
    pub fn name(mut self, value: QueryConsumersGetByIdResponseSourceOneName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryConsumersGetByIdResponseSourceOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryConsumersGetByIdResponseSourceOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryConsumersGetByIdResponseSourceOneBuilder::name)
    /// - [`r#type`](QueryConsumersGetByIdResponseSourceOneBuilder::r#type)
    pub fn build(self) -> Result<QueryConsumersGetByIdResponseSourceOne, BuildError> {
        Ok(QueryConsumersGetByIdResponseSourceOne {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

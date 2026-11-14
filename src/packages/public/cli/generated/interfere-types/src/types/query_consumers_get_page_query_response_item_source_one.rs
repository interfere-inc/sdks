pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryConsumersGetPageQueryResponseItemSourceOne {
    pub name: QueryConsumersGetPageQueryResponseItemSourceOneName,
    pub r#type: QueryConsumersGetPageQueryResponseItemSourceOneType,
}

impl QueryConsumersGetPageQueryResponseItemSourceOne {
    pub fn builder() -> QueryConsumersGetPageQueryResponseItemSourceOneBuilder {
        <QueryConsumersGetPageQueryResponseItemSourceOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryConsumersGetPageQueryResponseItemSourceOneBuilder {
    name: Option<QueryConsumersGetPageQueryResponseItemSourceOneName>,
    r#type: Option<QueryConsumersGetPageQueryResponseItemSourceOneType>,
}

impl QueryConsumersGetPageQueryResponseItemSourceOneBuilder {
    pub fn name(mut self, value: QueryConsumersGetPageQueryResponseItemSourceOneName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryConsumersGetPageQueryResponseItemSourceOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryConsumersGetPageQueryResponseItemSourceOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryConsumersGetPageQueryResponseItemSourceOneBuilder::name)
    /// - [`r#type`](QueryConsumersGetPageQueryResponseItemSourceOneBuilder::r#type)
    pub fn build(self) -> Result<QueryConsumersGetPageQueryResponseItemSourceOne, BuildError> {
        Ok(QueryConsumersGetPageQueryResponseItemSourceOne {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryConsumersGetPageQueryResponseItemSourceZero {
    pub name: QueryConsumersGetPageQueryResponseItemSourceZeroName,
    pub r#type: QueryConsumersGetPageQueryResponseItemSourceZeroType,
}

impl QueryConsumersGetPageQueryResponseItemSourceZero {
    pub fn builder() -> QueryConsumersGetPageQueryResponseItemSourceZeroBuilder {
        <QueryConsumersGetPageQueryResponseItemSourceZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryConsumersGetPageQueryResponseItemSourceZeroBuilder {
    name: Option<QueryConsumersGetPageQueryResponseItemSourceZeroName>,
    r#type: Option<QueryConsumersGetPageQueryResponseItemSourceZeroType>,
}

impl QueryConsumersGetPageQueryResponseItemSourceZeroBuilder {
    pub fn name(mut self, value: QueryConsumersGetPageQueryResponseItemSourceZeroName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryConsumersGetPageQueryResponseItemSourceZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryConsumersGetPageQueryResponseItemSourceZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryConsumersGetPageQueryResponseItemSourceZeroBuilder::name)
    /// - [`r#type`](QueryConsumersGetPageQueryResponseItemSourceZeroBuilder::r#type)
    pub fn build(self) -> Result<QueryConsumersGetPageQueryResponseItemSourceZero, BuildError> {
        Ok(QueryConsumersGetPageQueryResponseItemSourceZero {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

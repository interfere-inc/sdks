pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryConsumersGetByIdResponseSourceThree {
    #[serde(default)]
    pub name: String,
    pub r#type: QueryConsumersGetByIdResponseSourceThreeType,
}

impl QueryConsumersGetByIdResponseSourceThree {
    pub fn builder() -> QueryConsumersGetByIdResponseSourceThreeBuilder {
        <QueryConsumersGetByIdResponseSourceThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryConsumersGetByIdResponseSourceThreeBuilder {
    name: Option<String>,
    r#type: Option<QueryConsumersGetByIdResponseSourceThreeType>,
}

impl QueryConsumersGetByIdResponseSourceThreeBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: QueryConsumersGetByIdResponseSourceThreeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryConsumersGetByIdResponseSourceThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryConsumersGetByIdResponseSourceThreeBuilder::name)
    /// - [`r#type`](QueryConsumersGetByIdResponseSourceThreeBuilder::r#type)
    pub fn build(self) -> Result<QueryConsumersGetByIdResponseSourceThree, BuildError> {
        Ok(QueryConsumersGetByIdResponseSourceThree {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

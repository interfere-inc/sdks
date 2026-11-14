pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryConsumersGetByIdResponseSourceTwoType {
    #[serde(rename = "workos")]
    Workos,
}
impl fmt::Display for QueryConsumersGetByIdResponseSourceTwoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Workos => "workos",
        };
        write!(f, "{}", s)
    }
}

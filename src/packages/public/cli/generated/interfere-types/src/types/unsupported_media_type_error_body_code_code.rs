pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UnsupportedMediaTypeErrorBodyCodeCode {
    #[serde(rename = "AVATAR_UNSUPPORTED_CONTENT_TYPE")]
    AvatarUnsupportedContentType,
}
impl fmt::Display for UnsupportedMediaTypeErrorBodyCodeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AvatarUnsupportedContentType => "AVATAR_UNSUPPORTED_CONTENT_TYPE",
        };
        write!(f, "{}", s)
    }
}

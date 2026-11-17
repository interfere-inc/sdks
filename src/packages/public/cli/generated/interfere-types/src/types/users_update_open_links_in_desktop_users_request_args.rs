pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateOpenLinksInDesktopUsersRequestArgs {
    #[serde(rename = "openLinksInDesktop")]
    #[serde(default)]
    pub open_links_in_desktop: bool,
}

impl UpdateOpenLinksInDesktopUsersRequestArgs {
    pub fn builder() -> UpdateOpenLinksInDesktopUsersRequestArgsBuilder {
        <UpdateOpenLinksInDesktopUsersRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOpenLinksInDesktopUsersRequestArgsBuilder {
    open_links_in_desktop: Option<bool>,
}

impl UpdateOpenLinksInDesktopUsersRequestArgsBuilder {
    pub fn open_links_in_desktop(mut self, value: bool) -> Self {
        self.open_links_in_desktop = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateOpenLinksInDesktopUsersRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`open_links_in_desktop`](UpdateOpenLinksInDesktopUsersRequestArgsBuilder::open_links_in_desktop)
    pub fn build(self) -> Result<UpdateOpenLinksInDesktopUsersRequestArgs, BuildError> {
        Ok(UpdateOpenLinksInDesktopUsersRequestArgs {
            open_links_in_desktop: self.open_links_in_desktop.ok_or_else(|| BuildError::missing_field("open_links_in_desktop"))?,
        })
    }
}

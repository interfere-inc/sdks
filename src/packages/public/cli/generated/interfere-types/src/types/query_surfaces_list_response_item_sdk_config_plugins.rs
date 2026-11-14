pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuerySurfacesListResponseItemSdkConfigPlugins {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<bool>,
    #[serde(rename = "pageEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_events: Option<bool>,
    #[serde(rename = "rageClick")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rage_click: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<bool>,
}

impl QuerySurfacesListResponseItemSdkConfigPlugins {
    pub fn builder() -> QuerySurfacesListResponseItemSdkConfigPluginsBuilder {
        <QuerySurfacesListResponseItemSdkConfigPluginsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySurfacesListResponseItemSdkConfigPluginsBuilder {
    errors: Option<bool>,
    device: Option<bool>,
    page_events: Option<bool>,
    rage_click: Option<bool>,
    replay: Option<bool>,
    logs: Option<bool>,
}

impl QuerySurfacesListResponseItemSdkConfigPluginsBuilder {
    pub fn errors(mut self, value: bool) -> Self {
        self.errors = Some(value);
        self
    }

    pub fn device(mut self, value: bool) -> Self {
        self.device = Some(value);
        self
    }

    pub fn page_events(mut self, value: bool) -> Self {
        self.page_events = Some(value);
        self
    }

    pub fn rage_click(mut self, value: bool) -> Self {
        self.rage_click = Some(value);
        self
    }

    pub fn replay(mut self, value: bool) -> Self {
        self.replay = Some(value);
        self
    }

    pub fn logs(mut self, value: bool) -> Self {
        self.logs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuerySurfacesListResponseItemSdkConfigPlugins`].
    pub fn build(self) -> Result<QuerySurfacesListResponseItemSdkConfigPlugins, BuildError> {
        Ok(QuerySurfacesListResponseItemSdkConfigPlugins {
            errors: self.errors,
            device: self.device,
            page_events: self.page_events,
            rage_click: self.rage_click,
            replay: self.replay,
            logs: self.logs,
        })
    }
}

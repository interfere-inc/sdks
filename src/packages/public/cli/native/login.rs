use std::ffi::OsString;
use std::io::Write;

use fern_cli_sdk::auth::{
    DeviceCodeLoginFlow, DynAuthProvider, LoginContext, LoginFlow, PkceLoginFlow,
};
use fern_cli_sdk::error::CliError;

const GRANT_DISABLED_MARKER: &str = "urn:ietf:params:oauth:grant-type:device_code' grant";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginMethod {
    Browser,
    Device,
}

/// Host signals that decide whether a browser redirect back to this machine can complete.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Host {
    pub remote_shell: bool,
    pub ci: bool,
    pub linux: bool,
    pub display: bool,
}

impl Host {
    pub fn detect() -> Self {
        Self::from_lookup(cfg!(target_os = "linux"), |name| std::env::var_os(name))
    }

    pub fn from_lookup(linux: bool, lookup: impl Fn(&str) -> Option<OsString>) -> Self {
        let set = |name: &str| lookup(name).is_some_and(|value| !value.is_empty());
        let ci = lookup("CI").is_some_and(|value| {
            let value = value.to_string_lossy().to_ascii_lowercase();
            !matches!(value.as_str(), "" | "0" | "false" | "no")
        });
        Self {
            remote_shell: set("SSH_TTY") || set("SSH_CONNECTION") || set("SSH_CLIENT"),
            ci,
            linux,
            display: set("DISPLAY") || set("WAYLAND_DISPLAY"),
        }
    }

    pub fn browser_reachable(&self) -> bool {
        !self.remote_shell && !self.ci && (!self.linux || self.display)
    }
}

pub fn select_method(force_device: bool, host: &Host) -> LoginMethod {
    if force_device || !host.browser_reachable() {
        LoginMethod::Device
    } else {
        LoginMethod::Browser
    }
}

/// One login flow per OAuth scheme that signs in through the browser redirect when this machine
/// can receive it and through the device authorization grant otherwise.
#[derive(Debug, Clone)]
pub struct InterfereLoginFlow {
    environment: String,
    browser: PkceLoginFlow,
    device: DeviceCodeLoginFlow,
}

impl InterfereLoginFlow {
    pub fn new(environment: &str, browser: PkceLoginFlow, device: DeviceCodeLoginFlow) -> Self {
        Self {
            environment: environment.to_owned(),
            browser,
            device,
        }
    }

    pub fn run_with(&self, ctx: &LoginContext, method: LoginMethod) -> Result<(), CliError> {
        match method {
            LoginMethod::Browser => self.browser.run(ctx),
            LoginMethod::Device => self
                .device
                .run(ctx)
                .map_err(|error| self.explain_device_error(error)),
        }
    }

    fn explain_device_error(&self, error: CliError) -> CliError {
        match error {
            CliError::Auth(message) if message.contains(GRANT_DISABLED_MARKER) => {
                CliError::Auth(format!(
                    "Device sign-in is not enabled for the {} Interfere OAuth application yet. \
                     Run `interfere auth login` on a machine with a browser, or set \
                     INTERFERE_ACCESS_TOKEN. ({message})",
                    self.environment
                ))
            }
            other => other,
        }
    }
}

impl LoginFlow for InterfereLoginFlow {
    fn flow_type(&self) -> &'static str {
        "pkce, device-code without a browser"
    }

    fn scheme_name(&self) -> &str {
        self.browser.scheme_name()
    }

    fn run(&self, ctx: &LoginContext) -> Result<(), CliError> {
        let host = Host::detect();
        let method = select_method(false, &host);
        if method == LoginMethod::Device && !ctx.no_browser {
            let mut err = std::io::stderr().lock();
            let _ = writeln!(
                err,
                "No browser can return to this machine; using device sign-in instead."
            );
        }
        self.run_with(ctx, method)
    }

    fn token_paste_url(&self) -> Option<&str> {
        LoginFlow::token_paste_url(&self.browser)
    }

    fn build_auth_provider(&self, cli_name: &str) -> Option<DynAuthProvider> {
        self.browser.build_auth_provider(cli_name)
    }
}

#[cfg(test)]
#[path = "login_test.rs"]
mod tests;

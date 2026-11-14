use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Instant;

use tokio::sync::Mutex;

use fern_cli_sdk::auth::{
    set_active_store, DeviceCodeLoginFlow, KeyringStore, LoginContext, MockKeyringStore,
    PkceLoginFlow, TokenBundle,
};
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::profiles::keyring_account;
use serde_json::json;
use wiremock::matchers::{body_string_contains, method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

use super::{select_method, Host, InterfereLoginFlow, LoginMethod};

fn host(vars: &[(&str, &str)], linux: bool) -> Host {
    let vars: HashMap<&str, &str> = vars.iter().copied().collect();
    Host::from_lookup(linux, |name| vars.get(name).map(std::ffi::OsString::from))
}

#[test]
fn browser_login_is_the_default_on_a_desktop() {
    let desktop = host(&[], false);
    assert_eq!(select_method(false, &desktop), LoginMethod::Browser);
    let linux_desktop = host(&[("DISPLAY", ":0")], true);
    assert_eq!(select_method(false, &linux_desktop), LoginMethod::Browser);
    let wayland = host(&[("WAYLAND_DISPLAY", "wayland-0")], true);
    assert_eq!(select_method(false, &wayland), LoginMethod::Browser);
}

#[test]
fn device_login_when_no_browser_can_return_to_this_machine() {
    assert_eq!(
        select_method(false, &host(&[("SSH_TTY", "/dev/pts/0")], false)),
        LoginMethod::Device
    );
    assert_eq!(
        select_method(
            false,
            &host(&[("SSH_CONNECTION", "10.0.0.2 1 10.0.0.1 22")], false)
        ),
        LoginMethod::Device
    );
    assert_eq!(
        select_method(false, &host(&[("CI", "true")], false)),
        LoginMethod::Device
    );
    assert_eq!(
        select_method(false, &host(&[("CI", "false")], false)),
        LoginMethod::Browser
    );
    assert_eq!(select_method(false, &host(&[], true)), LoginMethod::Device);
    assert_eq!(
        select_method(false, &host(&[("DISPLAY", "")], true)),
        LoginMethod::Device
    );
}

#[test]
fn the_device_flag_forces_device_login() {
    let desktop = host(&[], false);
    assert_eq!(select_method(true, &desktop), LoginMethod::Device);
}

fn flow_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn flow(server: &MockServer) -> InterfereLoginFlow {
    let issuer = server.uri();
    let resource = "https://mcp.preview.interfere.com/mcp";
    let browser = PkceLoginFlow::new("oauth-preview")
        .client_id("cli-client")
        .authorization_url(format!("{issuer}/oauth/authorize"))
        .token_url(format!("{issuer}/oauth/token"))
        .redirect_host("127.0.0.1")
        .redirect_ports([8484]);
    let device = DeviceCodeLoginFlow::new("oauth-preview")
        .client_id("cli-client")
        .device_authorization_url(format!("{issuer}/oauth/device_authorization"))
        .token_url(format!("{issuer}/oauth/token"))
        .scopes(["interfere:access", "offline_access"])
        .device_authorization_params([("resource", resource)])
        .token_params([("resource", resource)]);
    InterfereLoginFlow::new("preview", browser, device)
}

fn ctx() -> LoginContext {
    LoginContext {
        cli_name: "interfere-test".into(),
        no_browser: true,
    }
}

async fn mount_device_authorization(server: &MockServer, interval: u64) {
    Mock::given(method("POST"))
        .and(path("/oauth/device_authorization"))
        .and(body_string_contains("client_id=cli-client"))
        .and(body_string_contains(
            "scope=interfere%3Aaccess+offline_access",
        ))
        .and(body_string_contains(
            "resource=https%3A%2F%2Fmcp.preview.interfere.com%2Fmcp",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "device_code": "device-secret",
            "user_code": "ABCD-EFGH",
            "verification_uri": format!("{}/activate", server.uri()),
            "verification_uri_complete": format!("{}/activate?user_code=ABCD-EFGH", server.uri()),
            "expires_in": 600,
            "interval": interval
        })))
        .expect(1)
        .mount(server)
        .await;
}

#[tokio::test(flavor = "multi_thread")]
async fn device_poll_waits_through_pending_and_slow_down_then_stores_the_credential() {
    let _guard = flow_lock().lock().await;
    let server = MockServer::start().await;
    let store = Arc::new(MockKeyringStore::new());
    set_active_store(store.clone());
    mount_device_authorization(&server, 0).await;

    let polls = Arc::new(AtomicU32::new(0));
    let seen = polls.clone();
    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .and(body_string_contains(
            "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Adevice_code",
        ))
        .and(body_string_contains("device_code=device-secret"))
        .and(body_string_contains(
            "resource=https%3A%2F%2Fmcp.preview.interfere.com%2Fmcp",
        ))
        .respond_with(
            move |_: &Request| match seen.fetch_add(1, Ordering::SeqCst) {
                0 => ResponseTemplate::new(400)
                    .set_body_json(json!({ "error": "authorization_pending" })),
                1 => ResponseTemplate::new(400).set_body_json(json!({ "error": "slow_down" })),
                _ => ResponseTemplate::new(200).set_body_json(json!({
                    "access_token": "access-1",
                    "refresh_token": "refresh-1",
                    "expires_in": 3600
                })),
            },
        )
        .expect(3)
        .mount(&server)
        .await;

    let started = Instant::now();
    flow(&server)
        .run_with(&ctx(), LoginMethod::Device)
        .expect("device login should complete");

    assert_eq!(polls.load(Ordering::SeqCst), 3);
    assert!(
        started.elapsed().as_secs_f64() >= 1.0 + 1.0 + 6.0,
        "slow_down must add at least five seconds to the poll interval, took {:?}",
        started.elapsed()
    );
    let stored = store
        .get("interfere-test", &keyring_account("oauth-preview"))
        .unwrap()
        .expect("credential stored under the OAuth scheme");
    let bundle: TokenBundle = serde_json::from_str(&stored).unwrap();
    assert_eq!(bundle.access_token, "access-1");
    assert_eq!(bundle.refresh_token.as_deref(), Some("refresh-1"));
}

#[tokio::test(flavor = "multi_thread")]
async fn device_poll_stops_when_the_code_expires() {
    let _guard = flow_lock().lock().await;
    let server = MockServer::start().await;
    let store = Arc::new(MockKeyringStore::new());
    set_active_store(store.clone());
    mount_device_authorization(&server, 0).await;
    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({ "error": "expired_token" })))
        .expect(1)
        .mount(&server)
        .await;

    let error = flow(&server)
        .run_with(&ctx(), LoginMethod::Device)
        .expect_err("an expired device code must stop the poll");
    assert!(
        matches!(&error, CliError::Auth(message) if message.contains("expired")),
        "{error}"
    );
    assert!(store
        .get("interfere-test", &keyring_account("oauth-preview"))
        .unwrap()
        .is_none());
}

#[tokio::test(flavor = "multi_thread")]
async fn explains_when_clerk_has_not_enabled_the_grant() {
    let _guard = flow_lock().lock().await;
    let server = MockServer::start().await;
    set_active_store(Arc::new(MockKeyringStore::new()));
    Mock::given(method("POST"))
        .and(path("/oauth/device_authorization"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "invalid_grant",
            "error_description": "The requested OAuth 2.0 Client does not have the 'urn:ietf:params:oauth:grant-type:device_code' grant."
        })))
        .expect(1)
        .mount(&server)
        .await;

    let error = flow(&server)
        .run_with(&ctx(), LoginMethod::Device)
        .expect_err("a disabled grant must fail");
    assert!(
        matches!(&error, CliError::Auth(message)
            if message.starts_with("Device sign-in is not enabled for the preview Interfere OAuth application")
                && message.contains("interfere auth login")),
        "{error}"
    );
}

mod custom;
mod environment;
mod inspect;
mod login;

use std::sync::Arc;

use environment::Environment;
use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::auth::{ApiKeyAuth, AuthCredentialSource, AuthStrategy, BearerAuth};
use fern_cli_sdk::openapi::discovery::{
    GlobalParameter, GlobalParameterApplyMode, GlobalParameterLocation,
};
use fern_cli_sdk::openapi::OpenApiBinding;
use fern_cli_sdk::profiles::ProfilesConfig;

fn application() -> Result<CliApp, String> {
    let name = std::env::var("INTERFERE_ENVIRONMENT").unwrap_or_else(|_| "production".into());
    let environment = Environment::load(&name)?;
    let scheme = format!("oauth-{name}");
    let spec = environment.specification(
        include_str!("../generated/cli/interfere/openapi0.json"),
        &scheme,
    )?;
    let login = Arc::new(environment.login(&scheme));
    Ok(custom::register(
        CliApp::new("interfere")
            .description("Interfere API access for developers and coding agents")
            .profiles(ProfilesConfig::new())
            .login_flow(login.as_ref().clone())
            .auth(BearerAuth::new("accessToken").env("INTERFERE_ACCESS_TOKEN"))
            .auth(BearerAuth::new("apiKey").env("INTERFERE_SECRET_KEY"))
            .auth(
                ApiKeyAuth::new("publicKeyHeader").source(AuthCredentialSource::any(vec![
                    AuthCredentialSource::cli("public-key"),
                    AuthCredentialSource::from_env("INTERFERE_PUBLIC_KEY"),
                ])),
            )
            .global_parameter(GlobalParameter {
                name: "workspace".into(),
                location: GlobalParameterLocation::Path,
                target: "workspaceSlug".into(),
                env: Some("INTERFERE_WORKSPACE".into()),
                default: None,
                optional: true,
                apply: GlobalParameterApplyMode::Auto,
                parameter_name: Some("workspace".into()),
                docs: Some("Workspace slug".into()),
            })
            .binding(
                OpenApiBinding::new()
                    .spec(&spec)
                    .auth_strategy(AuthStrategy::Routing),
            ),
        login,
    ))
}

fn main() {
    match application() {
        Ok(app) => app.run(),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    }
}

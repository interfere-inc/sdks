use std::collections::HashMap;

use fern_cli_sdk::auth::{DeviceCodeLoginFlow, PkceLoginFlow};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::login::InterfereLoginFlow;

const SCOPES: [&str; 3] = ["interfere:access", "user:org:read", "offline_access"];

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    #[serde(skip)]
    name: String,
    client_id: String,
    issuer: String,
    resource: String,
    api_url: String,
    app_url: String,
}

impl Environment {
    pub fn load(name: &str) -> Result<Self, String> {
        let mut environments: HashMap<String, Self> =
            serde_json::from_str(include_str!("../fern/oauth.json"))
                .map_err(|error| error.to_string())?;
        let mut environment = environments.remove(name).ok_or_else(|| {
            format!("Unknown INTERFERE_ENVIRONMENT '{name}'. Use production or preview.")
        })?;
        environment.name = name.to_owned();
        Ok(environment)
    }

    pub fn login(&self, scheme: &str) -> InterfereLoginFlow {
        InterfereLoginFlow::new(
            &self.name,
            self.browser_login(scheme),
            self.device_login(scheme),
        )
    }

    fn browser_login(&self, scheme: &str) -> PkceLoginFlow {
        PkceLoginFlow::new(scheme)
            .client_id(&self.client_id)
            .authorization_url(format!("{}/oauth/authorize", self.issuer))
            .token_url(format!("{}/oauth/token", self.issuer))
            .scopes(SCOPES)
            .redirect_host("127.0.0.1")
            .redirect_ports([8484, 8485])
            .redirect_path("/callback")
            .success_redirect_url(format!("{}/auth/cli/success", self.app_url))
            .error_redirect_url(format!("{}/auth/cli/error", self.app_url))
            .authorization_params([("resource", &self.resource)])
            .token_params([("resource", &self.resource)])
            .refresh_params([("resource", &self.resource)])
    }

    fn device_login(&self, scheme: &str) -> DeviceCodeLoginFlow {
        DeviceCodeLoginFlow::new(scheme)
            .client_id(&self.client_id)
            .device_authorization_url(format!("{}/oauth/device_authorization", self.issuer))
            .token_url(format!("{}/oauth/token", self.issuer))
            .scopes(SCOPES)
            .device_authorization_params([("resource", &self.resource)])
            .token_params([("resource", &self.resource)])
            .refresh_params([("resource", &self.resource)])
    }

    pub fn specification(&self, source: &str, scheme: &str) -> Result<String, String> {
        let mut spec: Value = serde_json::from_str(source).map_err(|error| error.to_string())?;
        spec["servers"] = json!([{ "url": self.api_url }]);
        let security = spec["components"]["securitySchemes"]
            .as_object_mut()
            .ok_or("Missing security schemes")?;
        let bearer = security
            .remove("clerkSession")
            .ok_or("Missing Clerk security scheme")?;
        security.insert(scheme.to_owned(), bearer.clone());
        security.insert("accessToken".into(), bearer);
        expand_security(&mut spec, scheme)?;
        for path in spec["paths"]
            .as_object_mut()
            .ok_or("Missing paths")?
            .values_mut()
        {
            for method in [
                "get", "put", "post", "delete", "options", "head", "patch", "trace",
            ] {
                if let Some(operation) = path.get_mut(method) {
                    expand_security(operation, scheme)?;
                }
            }
        }
        serde_json::to_string(&spec).map_err(|error| error.to_string())
    }
}

fn expand_security(operation: &mut Value, scheme: &str) -> Result<(), String> {
    let Some(security) = operation.get_mut("security") else {
        return Ok(());
    };
    let requirements = security
        .as_array_mut()
        .ok_or("Invalid security requirements")?;
    let mut tokens = Vec::new();
    for requirement in requirements.iter_mut() {
        let object = requirement
            .as_object_mut()
            .ok_or("Invalid security requirement")?;
        if let Some(scopes) = object.remove("clerkSession") {
            let mut token = object.clone();
            token.insert("accessToken".into(), scopes.clone());
            tokens.push(Value::Object(token));
            object.insert(scheme.to_owned(), scopes);
        }
    }
    requirements.splice(0..0, tokens);
    Ok(())
}

#[cfg(test)]
#[path = "environment_test.rs"]
mod tests;

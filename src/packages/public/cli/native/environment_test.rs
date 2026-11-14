use super::Environment;
use serde_json::{json, Value};

#[test]
fn preserves_security_requirements_when_adding_token_auth() {
    let source = json!({
        "components": { "securitySchemes": { "clerkSession": { "type": "http", "scheme": "bearer" } } },
        "security": [{ "clerkSession": [], "secondFactor": ["verify"] }],
        "paths": {
            "/private": {
                "parameters": [],
                "summary": "Private operations",
                "post": { "security": [{ "apiKey": [] }, { "clerkSession": ["read"], "secondFactor": [] }] },
                "get": {}
            },
            "/public": { "get": { "security": [] } }
        }
    });
    let env = Environment::load("preview").unwrap();
    let output: Value = serde_json::from_str(
        &env.specification(&source.to_string(), "oauth-preview")
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        output["servers"][0]["url"],
        "https://api.preview.interfere.com"
    );
    assert_eq!(
        output["security"],
        json!([
            { "accessToken": [], "secondFactor": ["verify"] },
            { "oauth-preview": [], "secondFactor": ["verify"] }
        ])
    );
    assert_eq!(
        output["paths"]["/private"]["post"]["security"],
        json!([
            { "accessToken": ["read"], "secondFactor": [] },
            { "apiKey": [] },
            { "oauth-preview": ["read"], "secondFactor": [] }
        ])
    );
    assert!(output["paths"]["/private"]["get"].get("security").is_none());
    assert_eq!(output["paths"]["/public"]["get"]["security"], json!([]));
    assert!(output["components"]["securitySchemes"]
        .get("clerkSession")
        .is_none());
    assert_eq!(
        output["components"]["securitySchemes"]["accessToken"],
        source["components"]["securitySchemes"]["clerkSession"]
    );
}

#[test]
fn rejects_unknown_environments() {
    assert!(Environment::load("prodution").is_err());
}

#[test]
fn environments_use_distinct_oauth_clients_and_resources() {
    let production = Environment::load("production").unwrap();
    let preview = Environment::load("preview").unwrap();
    assert_ne!(production.client_id, preview.client_id);
    assert_ne!(production.issuer, preview.issuer);
    assert_ne!(production.resource, preview.resource);
    assert_ne!(production.api_url, preview.api_url);
}

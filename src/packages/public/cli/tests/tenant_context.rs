use std::process::Command;

use serde_json::{json, Value};

const TENANT_ID: &str = "00000000-0000-0000-0000-000000000001";

fn request(command: &str, body: &Value, tenant: bool) -> Value {
    let mut process = Command::new(env!("CARGO_BIN_EXE_interfere"));
    for (name, _) in std::env::vars().filter(|(name, _)| name.starts_with("INTERFERE_")) {
        process.env_remove(name);
    }
    process
        .args(["tenants", command, "--json", &body.to_string(), "--dry-run"])
        .env("INTERFERE_ENVIRONMENT", "production")
        .env("INTERFERE_ACCESS_TOKEN", "cli-test-token")
        .env("INTERFERE_WORKSPACE", "acme");
    if tenant {
        process.args(["--end-user-org-id", TENANT_ID]);
    }
    let output = process.output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn tenant_patches_preserve_tier_values_and_nulls() {
    for body in [
        json!({"tier": "Enterprise"}),
        json!({"tier": null}),
        json!({"tier": " Enterprise Plus "}),
    ] {
        let result = request("update-context", &body, true);
        assert_eq!(result["method"], "PATCH");
        assert_eq!(result["body"], body);
        assert_eq!(
            result["url"],
            format!("https://api.interfere.com/v3/workspaces/acme/tenants/{TENANT_ID}/context")
        );
        assert!(!result.to_string().contains("cli-test-token"));
    }
}

#[test]
fn bulk_tenant_patches_preserve_each_tenant_identifier() {
    let body = json!({"tenants": [{"endUserOrgId": TENANT_ID, "tier": "Enterprise"}]});
    let result = request("update-contexts", &body, false);
    assert_eq!(result["body"], body);
    assert_eq!(result["method"], "PATCH");
    assert_eq!(
        result["url"],
        "https://api.interfere.com/v3/workspaces/acme/tenants/context"
    );
}

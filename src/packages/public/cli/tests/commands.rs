use std::process::{Command, Output};

use serde_json::{json, Value};

fn cli(args: &[&str], environment: &str) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_interfere"));
    for (name, _) in std::env::vars().filter(|(name, _)| name.starts_with("INTERFERE_")) {
        command.env_remove(name);
    }
    command
        .args(args)
        .env("INTERFERE_ENVIRONMENT", environment)
        .env("INTERFERE_ACCESS_TOKEN", "cli-test-token")
        .env("INTERFERE_WORKSPACE", "acme")
        .output()
        .unwrap()
}

#[test]
fn reports_the_package_version() {
    let output = cli(&["--version"], "production");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap().trim(),
        concat!("interfere ", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn schema_discovery_does_not_require_operation_arguments() {
    let output = cli(&["problems", "get", "--schema"], "production");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    let schema: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(schema.to_string().contains("problemId"));
}

#[test]
fn dry_run_preserves_wire_names_and_uses_the_selected_environment() {
    let output = cli(
        &[
            "problems",
            "get",
            "--args.problem-id",
            "00000000-0000-0000-0000-000000000001",
            "--dry-run",
        ],
        "preview",
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    let request: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        request["url"],
        "https://api.preview.interfere.com/v3/workspaces/acme/queries/problems.getById"
    );
    assert_eq!(
        request["body"]["args"]["problemId"],
        "00000000-0000-0000-0000-000000000001"
    );
    assert!(!String::from_utf8(output.stdout)
        .unwrap()
        .contains("cli-test-token"));
}

#[test]
fn inspection_dry_run_lists_both_requests_without_contacting_the_server() {
    let output = cli(
        &[
            "problems",
            "inspect",
            "example",
            "--dry-run",
            "--base-url",
            "http://127.0.0.1:1",
        ],
        "production",
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    let plan: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(plan["requests"].as_array().unwrap().len(), 2);
    assert_eq!(plan["requests"][0]["operationId"], "problems_get");
    assert_eq!(plan["requests"][1]["operationId"], "findings_listByProblem");
    assert_eq!(
        plan["requests"][0]["body"],
        json!({ "args": { "problemId": "example" } })
    );
}

#[test]
fn validation_errors_have_a_nonzero_exit_and_machine_readable_output() {
    let output = cli(&["problems", "get", "--format", "json"], "production");
    assert!(!output.status.success());
    let error: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(error["error"]["message"].as_str().is_some());
}

#[test]
fn unknown_environment_fails_instead_of_falling_back_to_production() {
    let output = cli(&["--version"], "prodution");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Unknown INTERFERE_ENVIRONMENT"));
}

#[test]
fn login_help_documents_device_sign_in() {
    let output = cli(&["login", "--help"], "production");
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();
    assert!(help.contains("--device"), "{help}");
    assert!(help.contains("--no-browser"), "{help}");
    assert!(help.contains("SSH"), "{help}");
}

#[test]
fn shell_setup_examples_name_the_binary() {
    for command in ["completion", "man"] {
        let output = cli(&[command, "--help"], "production");
        assert!(output.status.success());
        let help = String::from_utf8(output.stdout).unwrap();
        assert!(!help.contains("<CLI>"), "{help}");
        assert!(help.contains(&format!("interfere {command}")), "{help}");
    }
}

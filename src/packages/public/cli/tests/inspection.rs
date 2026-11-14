use std::process::Command;

use serde_json::{json, Value};
use wiremock::{
    matchers::{body_json, header, method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn inspection_composes_responses_and_applies_output_queries() {
    let server = MockServer::start().await;
    let body = json!({ "args": { "problemId": "example" } });
    Mock::given(method("POST"))
        .and(path("/v3/workspaces/acme/queries/problems.getById"))
        .and(header("Authorization", "Bearer cli-test-token"))
        .and(body_json(&body))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({ "id": "example", "timeline": [] })),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v3/workspaces/acme/queries/findings.listByProblem"))
        .and(header("Authorization", "Bearer cli-test-token"))
        .and(body_json(&body))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([{ "id": "finding" }])))
        .expect(1)
        .mount(&server)
        .await;
    let output = Command::new(env!("CARGO_BIN_EXE_interfere"))
        .args([
            "problems",
            "inspect",
            "example",
            "--workspace",
            "acme",
            "--base-url",
            &server.uri(),
            "--format",
            "json",
            "--query",
            "{id: problem.id, findings: findings[*].id}",
        ])
        .env("INTERFERE_ENVIRONMENT", "production")
        .env("INTERFERE_ACCESS_TOKEN", "cli-test-token")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    let response: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        response,
        json!({ "id": "example", "findings": ["finding"] })
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn inspection_stops_after_an_api_error_without_reporting_partial_success() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v3/workspaces/acme/queries/problems.getById"))
        .respond_with(ResponseTemplate::new(403).set_body_json(json!({ "message": "Forbidden" })))
        .expect(1)
        .mount(&server)
        .await;
    let output = Command::new(env!("CARGO_BIN_EXE_interfere"))
        .args([
            "problems",
            "inspect",
            "example",
            "--workspace",
            "acme",
            "--base-url",
            &server.uri(),
            "--format",
            "json",
        ])
        .env("INTERFERE_ENVIRONMENT", "production")
        .env("INTERFERE_ACCESS_TOKEN", "cli-test-token")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let response: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(response.get("error").is_some());
    assert!(response.get("problem").is_none());
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

use std::process::Command;

#[test]
fn product_groups_are_discoverable_without_credentials() {
    for args in [
        vec!["audit", "list", "--schema"],
        vec!["keys", "public", "create", "--schema"],
        vec!["keys", "private", "create", "--schema"],
        vec!["billing", "invoices", "list", "--schema"],
        vec!["users", "invitations", "create", "--schema"],
        vec!["notifications", "settings", "mine", "--schema"],
        vec!["scim", "groups", "list", "--schema"],
        vec!["support", "threads", "list", "--schema"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_interfere"))
            .args(&args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap();
    }
}

#[test]
fn internal_groups_are_absent_from_discovery() {
    let output = Command::new(env!("CARGO_BIN_EXE_interfere"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();
    for name in [
        "fingerprint-user-daily",
        "entitlements",
        "subscriptions",
        "presence",
        "replay-chunks",
    ] {
        assert!(!help.contains(name), "Internal group leaked: {name}");
    }
}

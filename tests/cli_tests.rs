use std::process::Command;
use std::fs;

#[test]
fn test_cli_validate_valid_file() {
    let output = Command::new("./target/debug/concerto-validator")
        .args(&["validate", "--input", "metamodel.json"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("✅ metamodel.json: Valid"));
    assert!(stdout.contains("✅ All validations passed!"));
}

#[test]
fn test_cli_validate_invalid_json() {
    // Create a temporary invalid JSON file
    let invalid_content = r#"{ "invalid": "structure" }"#;
    fs::write("test_invalid_temp.json", invalid_content).expect("Failed to write test file");

    let output = Command::new("./target/debug/concerto-validator")
        .args(&["validate", "--input", "test_invalid_temp.json"])
        .output()
        .expect("Failed to execute command");

    // Clean up
    fs::remove_file("test_invalid_temp.json").ok();

    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("❌ test_invalid_temp.json"));
    assert!(stdout.contains("❌ 1 validation(s) failed"));
}

#[test]
fn test_cli_no_input_files() {
    let output = Command::new("./target/debug/concerto-validator")
        .args(&["validate"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No input files specified"));
}

#[test]
fn test_cli_help() {
    let output = Command::new("./target/debug/concerto-validator")
        .args(&["--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("A CLI tool for validating Concerto model JSON ASTs"));
    assert!(stdout.contains("validate"));
}

#[test]
fn test_cli_version() {
    let output = Command::new("./target/debug/concerto-validator")
        .args(&["--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("concerto-validator 0.1.0"));
}
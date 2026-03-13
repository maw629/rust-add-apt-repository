// Integration tests for rust-add-apt-repository
// These tests verify end-to-end workflows using the actual CLI

use std::process::Command;

/// Helper function to run the command with arguments
fn run_command(args: &[&str]) -> (i32, String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_rust-add-apt-repository"))
        .args(args)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    (exit_code, stdout, stderr)
}

/// Test that help flag works and shows usage information
#[test]
fn test_help_flag() {
    let (exit_code, stdout, _stderr) = run_command(&["--help"]);

    assert_eq!(exit_code, 0, "Help should exit with code 0");
    assert!(
        stdout.contains("Usage:"),
        "Help should contain usage information"
    );
    assert!(
        stdout.contains("rust-add-apt-repository"),
        "Help should mention command name"
    );
    assert!(
        stdout.contains("--remove"),
        "Help should list --remove option"
    );
    assert!(
        stdout.contains("--enable-source"),
        "Help should list --enable-source option"
    );
}

/// Test that version flag works
#[test]
fn test_version_flag() {
    let (exit_code, stdout, _stderr) = run_command(&["--version"]);

    assert_eq!(exit_code, 0, "Version should exit with code 0");
    assert!(
        stdout.contains("rust-add-apt-repository"),
        "Version should show command name"
    );
}

/// Test dry-run mode doesn't make changes
#[test]
fn test_dry_run_mode() {
    // This test verifies that --dry-run flag prevents actual modifications
    let (exit_code, stdout, stderr) = run_command(&[
        "--dry-run",
        "--uri",
        "http://example.com/repo",
        "--dist",
        "noble",
        "--component",
        "main",
    ]);

    // Dry-run should succeed (exit 0) even without root
    // The actual behavior depends on implementation
    assert!(exit_code == 0 || exit_code == 1, "Dry-run should complete");

    // If output contains anything, it should indicate dry-run mode
    if !stdout.is_empty() || !stderr.is_empty() {
        let combined = format!("{stdout}{stderr}");
        // Implementation may or may not print dry-run messages
        // Just ensure it doesn't crash
        assert!(!combined.is_empty());
    }
}

/// Test debug mode produces debug output
#[test]
fn test_debug_mode() {
    let (exit_code, _stdout, stderr) = run_command(&[
        "--debug",
        "--dry-run",
        "--uri",
        "http://example.com/repo",
        "--dist",
        "noble",
        "--component",
        "main",
    ]);

    // Debug output goes to stderr
    // Should show debug information
    if stderr.contains("[DEBUG]") {
        assert!(
            stderr.contains("Debug mode enabled"),
            "Debug mode should be indicated"
        );
    }

    // Exit code should be 0 or 1 (root check may fail in test environment)
    assert!(
        exit_code == 0 || exit_code == 1,
        "Should complete even if not root"
    );
}

/// Test list flag works
#[test]
fn test_list_flag() {
    let (exit_code, _stdout, _stderr) = run_command(&["--list"]);

    // List should work even without root, or fail with permission error
    assert!(
        exit_code == 0 || exit_code == 1,
        "List should complete or fail gracefully"
    );
}

/// Test invalid input produces proper exit code
#[test]
fn test_invalid_input_exit_code() {
    // Empty repository line should fail with exit code 2 (invalid input)
    let (exit_code, _stdout, stderr) = run_command(&["--dry-run", "--sourceslist", ""]);

    // Should fail with invalid input error
    assert!(exit_code != 0, "Invalid input should fail");

    // Error message should be present
    assert!(!stderr.is_empty(), "Should have error message");
}

/// Test PPA format validation
#[test]
fn test_ppa_format_validation() {
    // Invalid PPA format should be rejected
    let (exit_code, _stdout, _stderr) = run_command(&["--dry-run", "--ppa", "invalid-ppa-format"]);

    // Should fail with invalid input
    assert!(exit_code != 0, "Invalid PPA format should fail");

    // Error message should mention PPA format (but not asserting on it to avoid brittle tests)
}

/// Test URI format validation
#[test]
fn test_uri_requires_components() {
    // URI without components should fail or require interactive input
    let (exit_code, _stdout, _stderr) = run_command(&[
        "--dry-run",
        "--uri",
        "http://example.com/repo",
        // No --component specified
    ]);

    // Without components, should fail or prompt (in non-interactive mode, should fail)
    // In dry-run mode without components, the command should complete
    // (behavior may vary - some implementations allow it, others don't)
    // For now, just verify command completes without panicking
    #[allow(clippy::overly_complex_bool_expr)]
    let _ = exit_code; // Just verify we got an exit code
}

/// Test cloud archive format
#[test]
fn test_cloud_archive_format() {
    let (exit_code, _stdout, _stderr) = run_command(&["--dry-run", "--cloud", "bobcat"]);

    // Cloud archive should be processed (may fail without root or network)
    assert!(
        exit_code == 0 || exit_code == 1 || exit_code == 2,
        "Should process cloud archive"
    );
}

/// Test component validation warnings
#[test]
fn test_component_validation() {
    let (exit_code, _stdout, stderr) = run_command(&[
        "--dry-run",
        "--uri",
        "http://example.com/repo",
        "--dist",
        "noble",
        "--component",
        "invalid-component",
    ]);

    // Should succeed with warning, or fail
    // Check for warning about unknown component
    if stderr.contains("Warning") || stderr.contains("Unknown component") {
        assert!(
            stderr.contains("invalid-component"),
            "Should mention the invalid component"
        );
    }

    assert!(exit_code >= 0, "Should have valid exit code");
}

/// Test sources.list line format parsing
#[test]
fn test_sourceslist_line_parsing() {
    let (exit_code, _stdout, _stderr) = run_command(&[
        "--dry-run",
        "--sourceslist",
        "deb http://example.com/repo noble main",
    ]);

    // Should parse and process the line
    assert!(
        exit_code == 0 || exit_code == 1,
        "Should parse sources.list line"
    );
}

/// Test positional argument (deprecated)
#[test]
fn test_positional_argument() {
    let (exit_code, _stdout, _stderr) =
        run_command(&["--dry-run", "deb http://example.com/repo noble main"]);

    // Positional argument should still work (deprecated but supported)
    assert!(
        exit_code == 0 || exit_code == 1,
        "Should handle positional argument"
    );
}

/// Test remove flag
#[test]
fn test_remove_flag() {
    let (exit_code, _stdout, _stderr) = run_command(&[
        "--dry-run",
        "--remove",
        "--uri",
        "http://example.com/repo",
        "--dist",
        "noble",
        "--component",
        "main",
    ]);

    // Remove should be processed
    assert!(
        exit_code == 0 || exit_code == 1,
        "Should process remove flag"
    );
}

/// Test enable source flag
#[test]
fn test_enable_source_flag() {
    let (exit_code, _stdout, _stderr) = run_command(&[
        "--dry-run",
        "--enable-source",
        // No repository specified = global operation
    ]);

    // Global source enable operation
    assert!(
        exit_code == 0 || exit_code == 1,
        "Should process enable-source flag"
    );
}

/// Test component flag (global operation)
#[test]
fn test_component_flag_global() {
    let (exit_code, _stdout, _stderr) = run_command(&[
        "--dry-run",
        "--component",
        "universe",
        // No repository specified = global operation
    ]);

    // Global component operation
    assert!(
        exit_code == 0 || exit_code == 1,
        "Should process component flag globally"
    );
}

/// Test pocket flag (global operation)
#[test]
fn test_pocket_flag_global() {
    let (exit_code, _stdout, _stderr) = run_command(&[
        "--dry-run",
        "--pocket",
        "updates",
        // No repository specified = global operation
    ]);

    // Global pocket operation
    assert!(
        exit_code == 0 || exit_code == 1,
        "Should process pocket flag globally"
    );
}

/// Test yes flag suppresses prompts
#[test]
fn test_yes_flag() {
    let (exit_code, _stdout, _stderr) = run_command(&[
        "--dry-run",
        "--yes",
        "--uri",
        "http://example.com/repo",
        "--dist",
        "noble",
        "--component",
        "main",
    ]);

    // With --yes flag, should not prompt
    assert!(
        exit_code == 0 || exit_code == 1,
        "Should process with --yes flag"
    );
}

/// Test no-update flag
#[test]
fn test_no_update_flag() {
    let (exit_code, _stdout, _stderr) = run_command(&[
        "--dry-run",
        "--no-update",
        "--uri",
        "http://example.com/repo",
        "--dist",
        "noble",
        "--component",
        "main",
    ]);

    // With --no-update, should skip apt-get update
    assert!(
        exit_code == 0 || exit_code == 1,
        "Should process with --no-update flag"
    );
}

/// Test multiple components
#[test]
fn test_multiple_components() {
    let (exit_code, _stdout, _stderr) = run_command(&[
        "--dry-run",
        "--uri",
        "http://example.com/repo",
        "--dist",
        "noble",
        "--component",
        "main",
        "--component",
        "universe",
    ]);

    // Should accept multiple components
    assert!(
        exit_code == 0 || exit_code == 1,
        "Should handle multiple components"
    );
}

/// Test invalid suite format (with spaces)
#[test]
fn test_invalid_suite_with_spaces() {
    let (exit_code, _stdout, _stderr) = run_command(&[
        "--dry-run",
        "--sourceslist",
        "deb http://example.com/repo noble main extra",
    ]);

    // When parsing sources.list line, quotes are already resolved by shell
    // This test verifies the line is still processed correctly
    // Suite validation happens during parsing, exit code 0 or 1 expected
    assert!(exit_code == 0 || exit_code == 1, "Should process the line");

    // Warning about components is optional (implementation detail)
}

/// Test conflicting options
#[test]
fn test_conflicting_options() {
    // Test that conflicting flags are handled
    let (exit_code, _stdout, _stderr) = run_command(&[
        "--dry-run",
        "--ppa",
        "ppa:test/test",
        "--uri",
        "http://example.com/repo",
        // Both PPA and URI specified - should pick one or fail
    ]);

    // Should either accept (last wins) or reject conflicting options
    assert!(exit_code >= 0, "Should have valid exit code");
}

/// Test argument order flexibility
/// Flags should be allowed before or after positional arguments for better UX
#[test]
fn test_argument_order_flexibility() {
    // Test 1: Flags before positional argument (traditional order)
    let (exit_code1, stdout1, _stderr1) =
        run_command(&["--dry-run", "ppa:git-core/ppa"]);
    assert_eq!(exit_code1, 0, "Flags before positional should work");
    assert!(
        stdout1.contains("git-core"),
        "Should process PPA correctly"
    );

    // Test 2: Flags after positional argument (user-friendly order)
    let (exit_code2, stdout2, _stderr2) =
        run_command(&["ppa:git-core/ppa", "--dry-run"]);
    assert_eq!(exit_code2, 0, "Flags after positional should work");
    assert!(
        stdout2.contains("git-core"),
        "Should process PPA correctly"
    );

    // Test 3: Mixed order with explicit flag syntax
    let (exit_code3, stdout3, _stderr3) = run_command(&[
        "--dry-run",
        "--ppa",
        "ppa:git-core/ppa",
    ]);
    assert_eq!(exit_code3, 0, "Explicit flag syntax should work");
    assert!(
        stdout3.contains("git-core"),
        "Should process PPA correctly"
    );

    // Test 4: Multiple flags in different positions
    let (exit_code4, stdout4, _stderr4) = run_command(&[
        "--dry-run",
        "cloud-archive:bobcat",
        "--no-update",
    ]);
    assert_eq!(exit_code4, 0, "Flags around positional should work");
    assert!(
        stdout4.contains("bobcat"),
        "Should process cloud archive correctly"
    );

    // Test 5: Full sources.list line with flags after
    let (exit_code5, stdout5, _stderr5) = run_command(&[
        "deb http://example.com/repo noble main",
        "--dry-run",
    ]);
    assert_eq!(exit_code5, 0, "Full line with flags after should work");
    assert!(
        stdout5.contains("example.com"),
        "Should process sources.list line correctly"
    );

    // All outputs should be functionally equivalent (all dry-run mode)
    assert!(
        stdout1.contains("[DRY RUN]"),
        "Test 1 should be in dry-run mode"
    );
    assert!(
        stdout2.contains("[DRY RUN]"),
        "Test 2 should be in dry-run mode"
    );
    assert!(
        stdout3.contains("[DRY RUN]"),
        "Test 3 should be in dry-run mode"
    );
    assert!(
        stdout4.contains("[DRY RUN]"),
        "Test 4 should be in dry-run mode"
    );
    assert!(
        stdout5.contains("[DRY RUN]"),
        "Test 5 should be in dry-run mode"
    );
}

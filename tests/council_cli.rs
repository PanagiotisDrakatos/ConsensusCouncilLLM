use std::{fs, path::Path};

use assert_cmd::Command;
use tempfile::tempdir;

fn fixture_path(rel: &str) -> String {
    format!("fixtures/auth_validation_fix/{rel}")
}

fn dependency_fixture_path(rel: &str) -> String {
    format!("fixtures/dependency_upgrade/{rel}")
}

fn security_fixture_path(rel: &str) -> String {
    format!("fixtures/security_regression_prevented/{rel}")
}

#[test]
fn run_creates_required_artifacts() {
    let out_dir = tempdir().expect("tempdir");

    Command::cargo_bin("council")
        .expect("binary")
        .args([
            "run",
            "--task",
            &fixture_path("task.json"),
            "--policy",
            &fixture_path("policy.json"),
            "--out",
            out_dir.path().to_str().expect("utf8"),
        ])
        .assert()
        .success();

    for rel in [
        "evidence_graph.json",
        "adjudication.json",
        "patch_attestation.json",
        "review_summary.md",
        "replay_record.json",
        "final_patch.diff",
    ] {
        assert!(out_dir.path().join(rel).exists(), "missing {rel}");
    }

    assert!(out_dir.path().join("worktree").is_dir(), "missing worktree");
}

#[test]
fn verify_passes_on_valid_generated_run() {
    let out_dir = tempdir().expect("tempdir");

    Command::cargo_bin("council")
        .expect("binary")
        .args([
            "run",
            "--task",
            &fixture_path("task.json"),
            "--policy",
            &fixture_path("policy.json"),
            "--out",
            out_dir.path().to_str().expect("utf8"),
        ])
        .assert()
        .success();

    Command::cargo_bin("council")
        .expect("binary")
        .args(["verify", "--run", out_dir.path().to_str().expect("utf8")])
        .assert()
        .success();

    assert!(out_dir.path().join("verify_report.json").exists());
}

#[test]
fn verify_reports_missing_required_artifact() {
    let out_dir = tempdir().expect("tempdir");

    Command::cargo_bin("council")
        .expect("binary")
        .args([
            "run",
            "--task",
            &fixture_path("task.json"),
            "--policy",
            &fixture_path("policy.json"),
            "--out",
            out_dir.path().to_str().expect("utf8"),
        ])
        .assert()
        .success();

    let missing = out_dir.path().join("final_patch.diff");
    fs::remove_file(&missing).expect("remove artifact");

    let output = Command::cargo_bin("council")
        .expect("binary")
        .args(["verify", "--run", out_dir.path().to_str().expect("utf8")])
        .output()
        .expect("verify output");

    assert!(!output.status.success(), "verify should fail");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}\n{stderr}");
    assert!(
        combined.contains("MISSING_FILE"),
        "expected missing file reason code, got: {combined}"
    );
}

#[test]
fn sample_fixture_inputs_exist() {
    for rel in [
        "task.json",
        "policy.json",
        "fixture_manifest.json",
        "proposals.json",
        "critiques.json",
        "checks.json",
        "repo/src/auth/callback.ts",
        "repo/tests/auth/callback.test.ts",
        "patches/proposal-b.patch",
        "patches/proposal-b/src/auth/callback.ts",
        "patches/proposal-b/tests/auth/callback.test.ts",
    ] {
        assert!(
            Path::new(&fixture_path(rel)).exists(),
            "fixture is missing {rel}"
        );
    }

    for rel in [
        "task.json",
        "policy.json",
        "fixture_manifest.json",
        "proposals.json",
        "critiques.json",
        "checks.json",
        "repo/package.json",
        "repo/package-lock.json",
        "repo/tests/api-client.test.ts",
        "patches/proposal-b.patch",
        "patches/proposal-b/package.json",
        "patches/proposal-b/package-lock.json",
        "patches/proposal-b/tests/api-client.test.ts",
    ] {
        assert!(
            Path::new(&dependency_fixture_path(rel)).exists(),
            "dependency fixture is missing {rel}"
        );
    }

    for rel in [
        "task.json",
        "policy.json",
        "fixture_manifest.json",
        "proposals.json",
        "critiques.json",
        "checks.json",
        "repo/src/auth/callback.ts",
        "repo/tests/auth/callback.test.ts",
        "patches/proposal-b.patch",
        "patches/proposal-b/src/auth/callback.ts",
        "patches/proposal-b/tests/auth/callback.test.ts",
    ] {
        assert!(
            Path::new(&security_fixture_path(rel)).exists(),
            "security fixture is missing {rel}"
        );
    }
}

#[test]
fn run_applies_winning_patch_to_worktree() {
    let out_dir = tempdir().expect("tempdir");

    Command::cargo_bin("council")
        .expect("binary")
        .args([
            "run",
            "--task",
            &fixture_path("task.json"),
            "--policy",
            &fixture_path("policy.json"),
            "--out",
            out_dir.path().to_str().expect("utf8"),
        ])
        .assert()
        .success();

    let callback = fs::read_to_string(out_dir.path().join("worktree/src/auth/callback.ts"))
        .expect("read patched callback");
    let tests = fs::read_to_string(out_dir.path().join("worktree/tests/auth/callback.test.ts"))
        .expect("read patched tests");
    let diff = fs::read_to_string(out_dir.path().join("final_patch.diff")).expect("read patch");

    assert!(
        callback.contains("new URL(candidate, ALLOWED_REDIRECT_BASE)"),
        "patched callback missing normalized URL logic"
    );
    assert!(
        tests.contains("rejects malformed callback targets"),
        "patched tests missing malformed callback case"
    );
    assert!(
        diff.contains("+const ALLOWED_REDIRECT_BASE"),
        "patch diff missing expected addition"
    );
}

#[test]
fn dependency_fixture_runs_and_verifies() {
    let out_dir = tempdir().expect("tempdir");

    Command::cargo_bin("council")
        .expect("binary")
        .args([
            "run",
            "--task",
            &dependency_fixture_path("task.json"),
            "--policy",
            &dependency_fixture_path("policy.json"),
            "--out",
            out_dir.path().to_str().expect("utf8"),
        ])
        .assert()
        .success();

    Command::cargo_bin("council")
        .expect("binary")
        .args(["verify", "--run", out_dir.path().to_str().expect("utf8")])
        .assert()
        .success();

    let package_json = fs::read_to_string(out_dir.path().join("worktree/package.json"))
        .expect("read patched package json");
    let patch = fs::read_to_string(out_dir.path().join("final_patch.diff")).expect("read diff");

    assert!(
        package_json.contains("\"axios\": \"1.7.4\""),
        "dependency fixture did not apply the winning package version"
    );
    assert!(
        patch.contains("\"axios\": \"1.7.4\""),
        "dependency fixture diff missing expected version bump"
    );
}

#[test]
fn security_fixture_runs_and_verifies() {
    let out_dir = tempdir().expect("tempdir");

    Command::cargo_bin("council")
        .expect("binary")
        .args([
            "run",
            "--task",
            &security_fixture_path("task.json"),
            "--policy",
            &security_fixture_path("policy.json"),
            "--out",
            out_dir.path().to_str().expect("utf8"),
        ])
        .assert()
        .success();

    Command::cargo_bin("council")
        .expect("binary")
        .args(["verify", "--run", out_dir.path().to_str().expect("utf8")])
        .assert()
        .success();

    let callback = fs::read_to_string(out_dir.path().join("worktree/src/auth/callback.ts"))
        .expect("read secured callback");
    let summary =
        fs::read_to_string(out_dir.path().join("review_summary.md")).expect("read review summary");

    assert!(
        callback.contains("isAllowedRedirectPath"),
        "security fixture did not apply the guarded refactor"
    );
    assert!(
        summary.contains("proposal-b"),
        "security fixture summary missing winning proposal"
    );
}

# Demo Runbook

## Purpose

This runbook reproduces the current fixed governed-change demo.

The demo is intentionally narrow:
- one task
- one policy
- replayed proposals and critiques
- deterministic adjudication
- one replayed winning patch applied to a fixed local worktree

## Demo Fixture

- Fixture: `fixtures/auth_validation_fix/`
- Change class: `auth_security`
- Risk level: `high`
- Scenario: harden OAuth callback redirect validation

## Commands

If the repository directory does not allow Cargo to create `target/` locally, use a writable target directory such as `/tmp/consensuscouncil-target`.

```bash
CARGO_TARGET_DIR=/tmp/consensuscouncil-target cargo test
CARGO_TARGET_DIR=/tmp/consensuscouncil-target cargo run -- run --task fixtures/auth_validation_fix/task.json --policy fixtures/auth_validation_fix/policy.json --out demo/sample_run
CARGO_TARGET_DIR=/tmp/consensuscouncil-target cargo run -- verify --run demo/sample_run
```

Expected verify result:

```text
PASS
```

## Generated Artifacts

The run should create:
- `demo/sample_run/evidence_graph.json`
- `demo/sample_run/adjudication.json`
- `demo/sample_run/patch_attestation.json`
- `demo/sample_run/review_summary.md`
- `demo/sample_run/replay_record.json`
- `demo/sample_run/final_patch.diff`
- `demo/sample_run/worktree/`
- `demo/sample_run/verify_report.json`

## What the Demo Proves

The demo proves:
- policy selection exists
- structured evidence exists
- adjudication exists
- one-final-writer designation exists
- attestation exists
- one real patch path exists for the fixed fixture
- replay and verification exist

The demo does not prove:
- live provider execution
- generalized patch application
- merge integration
- benchmark superiority

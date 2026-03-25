# Quickstart

## Fastest Path

Run the fixed governed-change demo:

```bash
CARGO_TARGET_DIR=/tmp/consensuscouncil-target cargo run -- run --task fixtures/auth_validation_fix/task.json --policy fixtures/auth_validation_fix/policy.json --out demo/sample_run
CARGO_TARGET_DIR=/tmp/consensuscouncil-target cargo run -- verify --run demo/sample_run
```

Expected result:

```text
PASS
```

## What to Inspect

- `demo/sample_run/review_summary.md`
- `demo/sample_run/adjudication.json`
- `demo/sample_run/patch_attestation.json`
- `demo/sample_run/final_patch.diff`
- `demo/sample_run/worktree/`
- `demo/sample_run/verify_report.json`

## Truthful Scope

The current quickstart replays structured proposals and critiques from fixture files, then applies the winning replayed patch to a fixed local worktree. It is a real patch path for one controlled fixture, not a general-purpose patching pipeline.

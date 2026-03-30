# Tamper Detection Example

This bundle includes one broken artifact set under `tampered_missing_diff/`.

## What was changed

The directory was copied from a valid governed run and then `final_patch.diff` was removed.

## Verification result

Running:

```bash
CARGO_TARGET_DIR=/tmp/consensuscouncil-target cargo run -- verify --run releases/v0.1-evidence-bundle/tampered_missing_diff
```

produced:

```text
FAIL BROKEN_REFERENCE,MISSING_FILE
Error: BROKEN_REFERENCE,MISSING_FILE
```

The generated `verify_report.json` records:

- `status`: `FAIL`
- `reason_codes`: `BROKEN_REFERENCE`, `MISSING_FILE`

## Why this matters

The normal governed runs prove the happy path.

This directory proves the verifier is not just ceremonial. If a required artifact disappears, the exported evidence surface stops validating and the run fails trust checks.

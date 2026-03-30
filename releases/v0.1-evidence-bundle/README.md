# ConsensusCouncilLLM v0.1 Evidence Bundle

## What this is

This directory is a local reviewer bundle for three governed runs:

- `auth_validation_fix`
- `dependency_upgrade`
- `security_regression_prevented`

Each run contains:
- `evidence_graph.json`
- `adjudication.json`
- `patch_attestation.json`
- `review_summary.md`
- `replay_record.json`
- `verify_report.json`
- `final_patch.diff`
- `worktree/`

The bundle is intentionally local-first. The public repository exists separately; this workspace snapshot is not itself a git repository, so no tagged release was created from here.

## What the three fixtures demonstrate

- `auth_validation_fix`: policy-governed hardening of an OAuth redirect path
- `dependency_upgrade`: narrow dependency upgrade with lockfile-aware governance
- `security_regression_prevented`: a prevented bad merge where a “cleaner” refactor removed input validation and was rejected before merge trust was granted

## How to verify

From the project root:

```bash
CARGO_TARGET_DIR=/tmp/consensuscouncil-target cargo run -- verify --run releases/v0.1-evidence-bundle/runs/auth_validation_fix
CARGO_TARGET_DIR=/tmp/consensuscouncil-target cargo run -- verify --run releases/v0.1-evidence-bundle/runs/dependency_upgrade
CARGO_TARGET_DIR=/tmp/consensuscouncil-target cargo run -- verify --run releases/v0.1-evidence-bundle/runs/security_regression_prevented
```

Each should return `PASS`.

Use the JSON schemas under `schemas/` to validate the exported artifacts independently.

## How to consume it

This bundle is designed to be machine-readable.

- `adjudication.json` answers which proposal won and why
- `patch_attestation.json` ties the policy, evidence, and final writer to the patch path
- `verify_report.json` records the verifier outcome for each governed run

See:
- `WHY_THIS_PREVENTED_A_BAD_MERGE.md`
- `TAMPER_PROOF.md`

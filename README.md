# ConsensusCouncilLLM

**Policy-governed admission, evidence, and attestation for AI-generated code changes**

A narrow trust layer between AI-generated diffs and merge decisions.

## What This Is

ConsensusCouncilLLM is a local-first governance layer for AI-generated code changes.

Its job is to decide:
- what policy applies to a change
- what evidence must exist before a change is trusted
- which proposal survives adjudication
- who becomes the final writer
- what attestation and replay record are exported

This repository now contains an **honest thin-slice prototype**:
- a Rust CLI named `council`
- JSON schemas for policy, evidence, adjudication, attestation, and replay
- fixture manifests plus three replayable governed-change fixtures under `fixtures/`
- generated sample artifacts under `demo/sample_run/`, `demo/dependency_upgrade_run/`, and `demo/security_regression_prevented_run/`
- replayed winning patch candidates applied to fixed local worktrees
- a verifier command that checks artifact presence, schema validity, linked references, and available hashes
- a local evidence bundle under `releases/v0.1-evidence-bundle/`

## What This Is Not

- not a generic multi-agent framework
- not a terminal orchestration product
- not a “models debate and vote on code” toy
- not a hosted service
- not a web dashboard or IDE suite
- not a claim that the current prototype performs general autonomous patching or merge

## Why Now

Agentic coding is already strong enough to produce meaningful diffs. The weak layer is still **merge trust**:
- review depth is rarely tied to change risk
- selection rationale is often lost
- evidence is not exported in reusable form
- approval paths are hard to replay later

ConsensusCouncilLLM treats model execution as input and focuses on the missing governance layer above it.

## Current Governed Flow

The implemented v1 demo is intentionally narrow and truthful:

1. Load a task file and a policy file.
2. Replay structured proposals from fixture JSON.
3. Replay at least one critique pass from fixture JSON.
4. Apply deterministic adjudication rules.
5. Designate one final writer.
6. Emit:
   - `evidence_graph.json`
   - `adjudication.json`
   - `patch_attestation.json`
   - `review_summary.md`
   - `replay_record.json`
   - `final_patch.diff`
   - `worktree/`
7. Run `council verify` to validate the artifact set.

The demo applies a **replayed winning patch candidate** to a fixed local fixture repo. It is a real patch path for one controlled case, not a general patching engine.

## Current Implementation Status

Implemented now:
- file-based CLI prototype in `src/`
- machine-readable schemas in `schemas/`
- example policy in `policies/`
- fixed auth validation fixture in `fixtures/auth_validation_fix/`
- fixed security-regression fixture in `fixtures/security_regression_prevented/`
- replayable sample run in `demo/sample_run/`
- fixed local worktree plus `final_patch.diff` for the winning proposal
- install and quickstart docs
- submission-oriented NLnet draft and budget package
- local evidence bundle and a tampered-bundle fail example for reviewer inspection

Still future work:
- live provider adapters
- broader fixture corpus beyond the first three replayable cases
- comparative eval harness across governed and ungoverned baselines
- public release proof beyond the local evidence bundle

## Prototype Honesty

This repository intentionally avoids fake maturity claims.

The current prototype:
- is CLI-first only
- uses replayed proposal and critique inputs
- is designed to prove the governance layer exists in early form
- uses attestation vocabulary inspired by provenance systems, but does **not** claim SLSA or in-toto compliance

## One End-to-End Demo

Canonical demo:
- change type: `auth_security`
- fixture: redirect validation hardening for an OAuth callback
- winner: `proposal-b`
- final writer: `agent:security`
- patch output: `final_patch.diff` + applied `worktree/`
- verification: `PASS`

Second replayable fixture:
- change type: `dependency_upgrade`
- fixture: narrow axios patch upgrade with lockfile sync
- winner: `proposal-b`
- final writer: `agent:release`
- verification: `PASS`

Third replayable fixture:
- change type: `auth_security`
- fixture: `security_regression_prevented`
- winner: `proposal-b`
- rejected proposal: `proposal-a` with `INPUT_VALIDATION_REMOVED`
- verification: `PASS`

See:
- `DEMO_RUNBOOK.md`
- `demo/sample_run/`
- `demo/security_regression_prevented_run/`
- `releases/v0.1-evidence-bundle/`

## Repo Map

- `src/` -> Rust CLI prototype
- `schemas/` -> JSON Schema definitions
- `policies/` -> sample policy files
- `fixtures/auth_validation_fix/` -> fixed demo inputs
- `fixtures/dependency_upgrade/` -> second replayable governed fixture
- `fixtures/security_regression_prevented/` -> prevented-bad-merge security fixture
- `demo/sample_run/` -> auth/security sample outputs
- `demo/dependency_upgrade_run/` -> dependency-upgrade sample outputs
- `demo/security_regression_prevented_run/` -> prevented-regression sample outputs
- `releases/v0.1-evidence-bundle/` -> local evidence bundle plus tampered-bundle fail example
- `INTEGRATION_CONTRACTS_V1.md` -> embed path for hooks, CI, and MCP-style wrappers
- `POSITIONING_MATRIX_V1.md` -> reviewer-facing comparison against adjacent tool categories
- `DEMO_PROOF.md` -> current runnable proof surface
- `COMPARISON.md` -> short comparison against adjacent tool categories
- `ROADMAP.md` -> narrow funded direction for the next 6 months

## Canonical Read Order

1. `README.md`
2. `QUICKSTART.md`
3. `DEMO_PROOF.md`
4. `ARCHITECTURE.md`
5. `COMPARISON.md`
6. `ROADMAP.md`

Supporting strategy notes remain under `round2/`. Historical material remains under `archive/`.

## Next Funded Work

The next funded pass should stay narrow:
- expand policy coverage beyond the current three fixtures
- add a small comparative replay/eval pack
- generalize the patch path beyond replay-driven fixtures
- strengthen verifier coverage and release packaging

The next lift comes from **verifiability**, not from adding more agent magic.

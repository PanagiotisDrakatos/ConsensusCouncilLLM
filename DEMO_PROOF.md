# Demo Proof

Updated: 2026-03-31

This file is the short proof surface for the current ConsensusCouncilLLM prototype.

## Fresh local verification

Run on the local snapshot in `ConsensusCouncilLLM/`:

- `cargo build` -> PASS
- `cargo test` -> PASS

Current test counts:
- 3 unit tests
- 10 integration tests

## What is currently demonstrated

- A runnable Rust CLI: `council run`, `council verify`
- Three replayable governed-change fixtures:
  - `auth_validation_fix`
  - `dependency_upgrade`
  - `security_regression_prevented`
- Generated artifact bundles that include:
  - `evidence_graph.json`
  - `adjudication.json`
  - `patch_attestation.json`
  - `review_summary.md`
  - `replay_record.json`
  - `final_patch.diff`
  - `worktree/`

## What the verifier now checks

For generated runs, `council verify` checks:

- required artifact presence
- schema conformance
- cross-reference consistency
- required policy checks
- reviewer-role coverage against policy
- human-witness state consistency
- hashed outputs where available, including the generated worktree

## Negative-path proof

The current regression suite verifies that `council verify` fails when:

- the generated worktree is modified after the run
- a required check is changed from passing to failing
- the human-witness state is changed to an inconsistent value

## Scope honesty

This is still a thin-slice prototype.

What it proves today:
- the governed artifact layer exists
- the replayable patch path exists for three controlled fixture classes
- the verifier is meaningful on both happy-path and selected tamper/failure cases

What it does not claim:
- production maturity
- general autonomous patching
- full end-to-end provenance security
- final merge authorization

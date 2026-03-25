# Contributing to ConsensusCouncilLLM

## Current Repository Status

This repository is now an **early implementation + grant-packaging repository**.

It still is not a production platform. Contributions at this stage should focus on:
- policy and schema precision
- verifier quality
- replayable fixture quality
- documentation consistency
- scope discipline

## Current Contribution Priorities

Useful contributions right now are:
- tightening policy DSL examples
- improving attestation and replay artifact quality
- improving deterministic adjudication rules
- adding truthful fixed fixtures
- removing stale or contradictory wording
- keeping README, demo, and grant docs aligned

## What Not to Contribute Yet

This repository should not pretend to accept product-scale contributions such as:
- provider adapters
- hosted control-plane work
- dashboard or IDE suite work
- benchmark claims without fixture-backed evidence

Those belong in later stages, not in the current thin slice.

## Contribution Rules

- Prefer precision over ambition
- Prefer narrow, auditable scope over platform sprawl
- Do not reintroduce generic multi-agent framing as the project identity
- Do not add funding claims or score estimates outside `FINAL_CLEANUP_REPORT.md`
- Keep historical material in `archive/` and mark it clearly as superseded if edited

## Useful Files to Read First

1. `README.md`
2. `CONSENSUSCOUNCILLLM_NLNET_COMMONS_DRAFT.md`
3. `ARCHITECTURE.md`
4. `Documentation.md`
5. `FINAL_CLEANUP_REPORT.md`

## Blocking Gaps to Respect

Current gaps that contributors should not paper over:
- no generalized patch application yet
- no multi-fixture eval pack yet
- no live provider orchestration yet
- no production release workflow yet

These gaps should be handled explicitly, not hidden with placeholder build instructions.

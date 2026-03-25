# Architecture: ConsensusCouncilLLM

## Purpose

ConsensusCouncilLLM is architected as the **policy-governed admission, evidence, and attestation layer for AI-generated code changes**.

The architecture is intentionally narrower than a generic orchestration stack. The core question is not “how do multiple agents collaborate?” The core question is:

> how do you make an AI-generated code change reviewable, attestable, and safer to trust?

## Architecture Principle

Model execution is an input surface. Governance is the product.

The system is strongest when it owns:
- policy selection
- evidence capture
- deterministic adjudication
- final-writer designation
- attestation export
- replay and verification

It is weaker when it expands into:
- full agent orchestration
- UI suites
- hosted control planes
- provider breadth before trust artifacts are stable

## Implemented V1 Slice

The repository now contains a narrow implemented slice:

1. `council run`
   - accepts one task file
   - accepts one policy file
   - loads replayed proposals and critiques from fixture JSON
   - applies deterministic adjudication
   - emits governed-change artifacts

2. `council verify`
   - checks required artifact presence
   - validates JSON artifacts against local schemas
   - validates linked artifact references
   - validates available digests
   - validates proposal and attestation cross-references

This is a truthful early governance layer, not a production patching engine.

## Layer Model

### 1. Policy Layer

Input:
- task definition
- declared change class
- declared risk level

Decision:
- required review depth
- required artifacts
- required checks
- whether human witness is required

Implemented now:
- one machine-readable auth/security policy
- schema validation for policy documents

### 2. Evidence Layer

Records:
- task
- proposals
- critiques
- adjudication reference
- generated artifact references
- final writer

Implemented now:
- `evidence_graph.json`

### 3. Adjudication Layer

Purpose:
- reject inadmissible proposals
- preserve rejected proposals in the evidence trail
- choose one winning proposal
- designate one final writer

Implemented now:
- deterministic ranking by:
  - `policy_compliance_score`
  - `change_scope_score`
  - `test_impact_score`
  - lower `estimated_change_breadth`
  - stable proposal id ordering

### 4. Attestation Layer

Purpose:
- export a machine-readable record for the governed change decision
- preserve policy, checks, evidence references, and human witness state

Implemented now:
- `patch_attestation.json`
- SLSA / in-toto-inspired field names
- no claim of formal compliance

### 5. Replay Layer

Purpose:
- make the governed run reproducible
- record the fixed inputs behind the demo

Implemented now:
- `replay_record.json`
- fixed fixture-backed replay path

### 6. Verification Layer

Purpose:
- prove the governed artifact set is internally consistent

Implemented now:
- `verify_report.json`
- PASS/FAIL reason codes:
  - `MISSING_FILE`
  - `SCHEMA_INVALID`
  - `BROKEN_REFERENCE`
  - `ATTESTATION_FIELD_MISSING`
  - `DIGEST_MISMATCH`

## Canonical V1 Artifact Flow

1. Load task and selected policy.
2. Load replayed proposals and critiques.
3. Reject inadmissible proposals.
4. Select one winner and one final writer.
5. Emit evidence graph, adjudication, attestation, summary, and replay record.
6. Verify the run.

## What Exists Now vs Later

Exists now:
- CLI prototype
- schemas
- sample policy
- fixed governed demo
- fixed patch-path binding for the auth fixture
- verification pass
- install and packaging docs

Later funded work:
- more policy classes
- more fixtures
- broader patch-path binding
- broader eval corpus
- stronger release packaging

## Non-Goals

- not a full orchestration platform
- not a provider zoo
- not a merge bot with autonomous authority
- not a web dashboard
- not a generalized “agent council” product

## Active Architectural Reading

The active reading across the repository should remain:

- structured consensus can be a useful substrate
- ConsensusCouncilLLM is the governance layer above that substrate
- the trust lift comes from schemas, artifacts, verifier behavior, and replayability

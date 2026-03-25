# Reviewer Walkthrough

**Purpose:** Understand the full ConsensusCouncilLLM flow in under 60 seconds

## The flow

```
Risk class (e.g., auth/security change)
     ↓
Policy gate (auth_security_v1.json) → admits or rejects based on change class
     ↓
Evidence capture (proposals, critiques, checks) → all recorded as JSON artifacts
     ↓
Deterministic adjudication → scores proposals, selects winner, rejects non-compliant
     ↓
Final writer designation → one responsible identity for the patch
     ↓
Attestation (patch_attestation.json) → machine-readable merge record
     ↓
Verification (council verify) → PASS/FAIL on artifact presence, schema, references, digests
     ↓
Merge trust: the decision is auditable, replayable, and externally verifiable
```

## What it does in three sentences

ConsensusCouncilLLM turns an AI-generated code change from an opaque suggestion into a governed decision with a machine-readable evidence trail.

The attestation answers the post-merge question: **why was this change accepted, what was checked, and what was rejected?**

The verifier answers the integrity question: **is the evidence trail internally consistent and complete?**

## Concrete example: security_regression_prevented

In the `security_regression_prevented` demo:

1. The policy required auth/security-class review depth.
2. Two proposals were submitted.
3. Proposal A removed redirect validation to make the callback path “simpler.”
4. Proposal B kept validation intact and extracted the check into a smaller guard helper.
5. Proposal A was rejected with `INPUT_VALIDATION_REMOVED`.
6. Proposal B was selected as winner and exported as the final patch path.
7. The verifier confirms: all artifacts present, schemas valid, references resolve, digests match → **PASS**.

The entire flow is replayable: running `council run` on the same fixture inputs produces the same artifacts, and `council verify` produces the same PASS result.

## What this means for NLnet reviewers

ConsensusCouncilLLM does not claim to improve AI code quality. It claims to make AI-generated code changes **governable**: policy-gated, evidence-traced, attestable, and verifiable. This is the missing infrastructure layer between AI code generation tools (Copilot, Cursor, Codex) and CI/CD quality gates (SLSA, in-toto, Sigstore).

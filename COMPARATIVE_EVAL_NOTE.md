# Comparative Evaluation Note

**Date:** 2026-03-11
**Purpose:** Show why the governed flow is meaningfully better than alternatives

## Evaluation paths

Three paths are compared for the same AI-generated code change:

- **Path A — Ungoverned (generate and inspect):** A developer receives a raw AI-generated patch. No policy check, no adjudication, no attestation. The developer must manually inspect the diff and decide whether to merge.
- **Path B — Single review without attestation:** The patch goes through one round of LLM critique (one model reviews, flags issues). No policy layer, no structured evidence, no machine-readable attestation.
- **Path C — ConsensusCouncilLLM governed path:** The full flow: policy admission → multi-proposal evidence → deterministic adjudication → attestation → verifier.

## Fixture comparison

| Fixture | Path | Policy gate | Structured evidence | Attestation | Verifiable | Unsafe merge risk |
|---|---|---|---|---|---|---|
| auth_validation_fix | A: ungoverned | none | none | none | no | **high** — reviewer may miss scope creep in a security-sensitive change |
| auth_validation_fix | B: single review | none | partial (one critique, unstructured) | none | no | **medium** — one reviewer can miss a policy violation in the auth layer |
| auth_validation_fix | C: governed | yes (auth/security policy) | full (proposals, critiques, adjudication) | yes (patch_attestation.json) | yes (council verify → PASS) | **low** — policy-inadmissible proposals are rejected before merge decision |
| dependency_upgrade | A: ungoverned | none | none | none | no | **medium** — version bump may introduce transitive vulnerability |
| dependency_upgrade | B: single review | none | partial (one critique) | none | no | **medium** — single reviewer may not check transitive dependency impact |
| dependency_upgrade | C: governed | yes (dependency policy) | full (proposals, critiques, adjudication) | yes (patch_attestation.json) | yes (council verify → PASS) | **low** — scope and compliance are structurally checked against policy |
| security_regression_prevented | A: ungoverned | none | none | none | no | **high** — a “cleaner” refactor can silently remove validation |
| security_regression_prevented | B: single review | none | partial (one critique) | none | no | **medium-high** — reviewer text can flag the issue but leaves no durable merge-trust record |
| security_regression_prevented | C: governed | yes (auth/security policy) | full (proposals, critiques, adjudication) | yes (patch_attestation.json) | yes (council verify → PASS) | **low** — `proposal-a` is rejected before merge because it removes input validation |

## Analysis

In Path A, the merge decision is fully manual. There is no record of *why* the patch was accepted, and no machine-readable evidence exists post-merge. If the change later causes an incident, there is no artifact trail connecting the AI-generated suggestion to the review decision. The developer carries the full cognitive burden of evaluating whether the change is safe for the declared risk class.

Path B improves on A by adding one critique round, but the critique is unstructured text, there is no policy admission gate, and the evidence trail is not machine-readable or verifiable. A single reviewer can miss a subtle scope violation — especially in security-sensitive changes where the diff looks locally reasonable but violates a higher-level policy constraint. There is no attestation, no replay capability, and no way for a second reviewer to verify the first review without repeating it.

Path C produces a complete governance record. If the patch is accepted, there is a machine-verifiable attestation linking the policy, the evidence, the adjudication decision, and the final patch. If a reviewer later asks "why was this merged?", the answer is in the artifact set, not in chat logs or memory. The verifier (`council verify`) independently confirms that all artifacts are present, schemas are valid, references resolve, and digests match.

The `security_regression_prevented` fixture is the clearest necessity proof. The rejected proposal is not broad or obviously malicious. It looks like the kind of tidy refactor a reviewer could wave through. The governed path blocks it because the policy binds the change to a security-sensitive admission standard, and the evidence trail records exactly why the merge trust level dropped.

## Key insight

The value of ConsensusCouncilLLM is not that it produces better patches. It is that it makes the merge decision **auditable, attestable, and replayable** — which is what is currently missing between AI code generation and production trust.

The design choice to use structured scoring rather than majority vote is supported by recent work showing that evidence-based adjudication outperforms frequency-based voting, especially when a minority proposal is correct but outvoted (cf. AgentAuditor, arXiv:2602.09341). The choice to use independent-proposal voting rather than iterative consensus is robust for tasks where diversity of approaches matters more than convergence (cf. ACL 2025 Findings, arXiv:2502.19130).

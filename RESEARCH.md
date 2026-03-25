# Research and Category Notes

## Purpose of This File

This file is a grounded category note for ConsensusCouncilLLM.

It is not meant to be a speculative literature dump and it should not be treated as a source of marketing claims. Its job is to explain the category the project belongs to, the adjacent categories it should avoid being collapsed into, and the evaluation questions that still matter.

## The Category Shift

The critical category shift is:

- **wrong category:** generic multi-agent coding framework
- **correct category:** governance and evidence layer for AI-generated code changes

This matters because execution tooling is already crowded. The stronger defensible claim is not “multiple agents can collaborate.” The stronger claim is that open ecosystems still lack a clean way to govern trust around AI-generated code changes.

## What Existing Ecosystems Already Cover

Current coding-agent ecosystems and operator setups already cover meaningful parts of:
- agent execution
- worktree isolation
- hooks and tool invocation
- plan and review loops
- answer comparison

That means ConsensusCouncilLLM should not be judged on whether it can reproduce those primitives. It should be judged on what it adds above them.

## What the Current Repository Now Shows

The repository now demonstrates a truthful first governance slice:
- policy schema and sample policy
- structured proposals and critiques
- deterministic adjudication
- patch attestation export
- replay record export
- verification pass over the artifact set

This does not prove a full governed patching system, but it does prove that the governance layer can exist as concrete local infrastructure.

## The Missing Layer

The missing layer is governance, not execution.

The project becomes interesting only if it can answer:
- what policy applies to this change
- what evidence was considered
- who wrote the final patch
- who reviewed it
- why the patch was accepted
- whether the process can be replayed later

That is why the key missing pieces remain:
- broader policy coverage
- real patch-path binding
- multi-fixture replay and evaluation harness
- public release packaging

## Adjacent Categories and Why They Are Not Enough

### Agent execution tools

These help run agents, but they do not solve governed admission of AI-generated change.

### Worktree-isolation tools

These help isolate work, but isolation alone does not explain why a patch should be trusted.

### Review bots

These help after code exists, but they do not preserve the structured decision process that led to the chosen patch.

### Answer panels

These help compare responses, but answer comparison is weaker than governed code-change attestation.

## Evaluation Implications

The evaluation problem is broader than “did a patch compile.”

A serious benchmark plan should compare:
- direct single-agent patching
- panel-based selection without governance binding
- governed change flow with policy, evidence, and attestation

The task mix should include:
- bug-fix work
- review-fix work
- test-generation work
- style or hygiene corrections
- longer-horizon feature work

The repository currently includes only the first fixed governed fixture:
- `auth_validation_fix`

That is enough to prove the demo path, but not enough to support comparative performance claims.

## Open Questions That Still Matter

1. How small can the policy DSL stay while still being useful?
2. What evidence is actually decision-useful rather than verbose?
3. What is the minimum attestation record that helps a maintainer trust a patch?
4. When is consensus worth running and when is direct execution enough?
5. What governed workflow produces the best balance of quality, cost, and reviewer effort?

## Current Research Boundary

This repository should stay disciplined:
- it should not rely on inflated “first” claims
- it should not rely on generic multi-agent novelty
- it should not rely on speculative citations as its main proof

The strongest active research claim is narrower:

> consensus mechanics are already feasible; the open problem is how to turn them into governed, attestable, replayable code-change infrastructure

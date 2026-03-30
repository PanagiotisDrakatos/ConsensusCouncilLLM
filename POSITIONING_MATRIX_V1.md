# ConsensusCouncilLLM Positioning Matrix v1

## Current Thesis

ConsensusCouncilLLM is the policy-governed admission, evidence, and attestation layer for AI-generated code changes.

It should be read as a narrow trust layer between generated change and merge trust.

## Comparison Matrix

| Category | Typical examples | What they do well | What they do not provide by default | ConsensusCouncilLLM position |
|---|---|---|---|---|
| Multi-agent frameworks | AutoGen, CrewAI, LangGraph | orchestration, routing, tool use, agent coordination | policy-bound admission, exported adjudication artifacts, patch attestation, replayable merge-trust records | complementary governance layer above model execution |
| CI/code scanning tools | SonarQube, Snyk, code scanning | findings, static checks, policy gates on repository state | proposal selection, final-writer designation, change-specific evidence graph for AI-generated diffs | complementary review/evidence layer |
| Provenance / attestation standards | SLSA, in-toto | artifact provenance vocabulary, attestations, verification language | task-level adjudication over competing AI-generated proposals | vocabulary inspiration, not a direct competitor |

## Reviewer Shortcut

The simplest correct mental model is:

- not another agent framework
- not another scanner
- not another provenance standard
- a small governance layer that sits between generated change and merge trust

## Current Proof

The repository currently proves this positioning with:
- schemas
- replayable fixtures
- adjudication output
- patch attestation
- verifier-backed worktree output

It does **not** yet prove:
- broad production integrations
- large fixture coverage
- live provider orchestration

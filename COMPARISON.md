# Comparison with Existing Approaches

ConsensusCouncilLLM targets a specific gap: **governance of AI-generated code changes**. This is distinct from code generation, artifact signing, or multi-agent orchestration.

## Comparison Matrix

| Capability | ConsensusCouncilLLM | SLSA / in-toto | Sigstore | Copilot / Cursor | Multi-agent panels |
|------------|:---:|:---:|:---:|:---:|:---:|
| Policy by change class | Yes | No | No | No | No |
| Structured evidence for proposals | Yes | No | No | No | Partial |
| Deterministic adjudication | Yes | No | No | No | Partial (voting) |
| One-final-writer discipline | Yes | No | No | N/A | No |
| Attestation export | Yes | Yes | Yes | No | No |
| Replay and verification | Yes | Partial | Partial | No | No |
| Governs AI-generated change | Yes | No | No | No | No |
| Local-first, file-based | Yes | Partial | No (online) | No (cloud) | Varies |

## What Each Tool Does Well

### SLSA / in-toto
Build provenance and supply-chain integrity. Excellent at proving *how a binary was produced*. Does not govern what review happened before a code change was accepted.

### Sigstore (Cosign, Rekor, Fulcio)
Keyless code signing and transparency logs. Proves *who signed what*. Does not evaluate whether a change was reviewed, by what policy, or with what evidence.

### Copilot / Cursor / AI Coding Assistants
Code generation and suggestion. Produce useful diffs. Do not provide structured governance over whether those diffs should be merged, under what policy, or with what attestation.

### Multi-agent Debate Panels (Claude-Octopus, Ruflo, OpenCode Teams, etc.)
Agent orchestration and answer selection. Choose which model produces the best answer. Do not provide policy-governed admission, evidence trails, or attestation for merge decisions.

## Where ConsensusCouncilLLM Fits

```
Code Generation          Governance Layer         Artifact Signing
(Copilot, Cursor)   -->  (ConsensusCouncilLLM) --> (SLSA, Sigstore)
                              |
                         Policy + Evidence
                         + Adjudication
                         + Attestation
                         + Replay/Verify
```

ConsensusCouncilLLM is not a replacement for any of these tools. It fills the gap between code generation and artifact signing — the governance layer that decides whether a specific AI-generated change is safe to merge.

## Research Support

The design choice to use structured scoring rather than majority vote is supported by:
- AgentAuditor (arXiv:2602.09341): evidence-based adjudication outperforms frequency-based voting
- ACL 2025 Findings (arXiv:2502.19130): independent-proposal voting is more robust when diversity matters

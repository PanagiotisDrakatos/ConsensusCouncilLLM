# Research Foundations

How the applicant's peer-reviewed research informs ConsensusCouncilLLM's design.

## Design decisions grounded in published work

### 1. Consensus protocol design → Adjudication layer

**Paper:** "Adrestus: Secure, scalable blockchain technology in a decentralized ledger via zones" (Elsevier, 2022)
DOI: 10.1016/j.bcra.2022.100093

**Connection:** The zone-based consensus architecture in Adrestus demonstrated that consensus can be scoped to specific change domains rather than applied globally. This directly informed ConsensusCouncilLLM's policy-by-change-class model: different change types (auth/security, dependency upgrade, refactoring) receive different governance policies, analogous to how blockchain zones process different transaction types under different consensus rules.

### 2. Data decaying and retention → Evidence lifecycle

**Paper:** "A blockchain datastore for scalable IoT workloads using data decaying" (Springer, 2024)
DOI: 10.1007/s10619-024-07441-9

**Connection:** Research on time-bounded data retention in distributed systems informed the evidence graph design. Governance artifacts have natural lifecycle boundaries: active evidence during review, archived evidence after merge, and retention policies for audit trails. The data decaying model validated the approach of structured, time-aware artifact management rather than unbounded log accumulation.

### 3. Scalable data management → Schema design

**Paper:** "Blockchain Data Management for IoT Applications" (IEEE MDM, 2022)
DOI: 10.1109/MDM55031.2022.00076

**Connection:** Experience designing schemas for high-throughput distributed data management directly shaped the JSON schema architecture: compact, machine-readable, cross-referenceable artifacts. The schema validation approach (verifier checks) mirrors blockchain state validation patterns.

### 4. Database architecture for distributed workloads → Verification layer

**Paper:** "Towards a Blockchain Database for Massive IoT Workloads" (IEEE ICDE Workshops, 2021)
DOI: 10.1109/ICDEW53142.2021.00021

**Connection:** ICDE research on database integrity verification informed the verifier's design: check artifact presence, validate schema conformance, verify cross-references, and detect digest mismatches. The verifier operates as a local state validator, conceptually similar to blockchain state root verification.

### 5. Transaction assignment and load distribution → Policy routing

**Paper:** "Rapid Blockchain Scaling with Efficient Transaction Assignment" (IEEE SEEDA-CECNSM, 2021)
DOI: 10.1109/SEEDA-CECNSM53056.2021.9566222

**Connection:** Research on efficient transaction routing to appropriate processing zones informed the policy layer's change-class routing. The system assigns governance depth based on change classification, paralleling how blockchain systems assign transactions to zones based on type and load characteristics.

### 6. Triastore: Web 3.0 data architecture → Attestation format

**Paper:** "Triastore: A Web 3.0 Blockchain Datastore for Massive IoT Workloads" (IEEE MDM, 2021)
DOI: 10.1109/MDM52706.2021.00038

**Connection:** The Triastore architecture's emphasis on structured, verifiable data records influenced the attestation format design: each attestation links back to its evidence, adjudication, and policy artifacts through content-addressable references, enabling independent verification.

### 7. Security architecture and threat modeling → Threat model

**Paper:** "Botnet Command and Control Architectures Revisited: Tor Hidden Services and Fluxing" (WISE 2017, Springer LNCS)
DOI: 10.1007/978-3-319-68786-5_41

**Connection:** Research on adversarial architectures and command-and-control resilience informed the threat model. Understanding how adversaries structure attack infrastructure helped design the governance layer's trust boundaries: model outputs are treated as untrusted inputs, attestation forgery requires producing a consistent artifact set, and the verifier serves as an independent trust anchor.

## Cite this project

```bibtex
@software{consensuscouncilllm2026,
  author = {Drakatos, Panagiotis},
  title = {ConsensusCouncilLLM: Policy-governed admission, evidence, and attestation for AI-generated code changes},
  year = {2026},
  url = {https://github.com/PanagiotisDrakatos/ConsensusCouncilLLM},
  license = {MIT}
}
```

## Note on research framing

These connections are presented to show how the applicant's technical background reduces execution risk — not to claim that ConsensusCouncilLLM is a direct research output of these papers. The governance layer is an engineering project informed by research experience, not a research paper itself.

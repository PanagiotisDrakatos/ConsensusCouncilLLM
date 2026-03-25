use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};

use crate::schema::{
    read_json_file, resolve_repo_path, sha256_for_file, validate_json_file_against_schema,
    AdjudicationResult, EvidenceGraph, PatchAttestation, Policy, ReplayRecord, VerifyReport,
};

const REQUIRED_FILES: [&str; 7] = [
    "evidence_graph.json",
    "adjudication.json",
    "patch_attestation.json",
    "review_summary.md",
    "replay_record.json",
    "final_patch.diff",
    "worktree",
];

pub fn execute_verify(run_dir: &Path) -> Result<VerifyReport> {
    let mut checked_files = Vec::new();
    let mut reason_codes = BTreeSet::new();

    for rel in REQUIRED_FILES {
        let path = run_dir.join(rel);
        checked_files.push(rel.to_string());
        if !path.exists() {
            reason_codes.insert(String::from("MISSING_FILE"));
        }
    }

    let evidence_graph_path = run_dir.join("evidence_graph.json");
    let adjudication_path = run_dir.join("adjudication.json");
    let patch_attestation_path = run_dir.join("patch_attestation.json");
    let replay_record_path = run_dir.join("replay_record.json");

    if evidence_graph_path.exists()
        && validate_json_file_against_schema("evidence_graph.schema.json", &evidence_graph_path)
            .is_err()
    {
        reason_codes.insert(String::from("SCHEMA_INVALID"));
    }
    if adjudication_path.exists()
        && validate_json_file_against_schema("adjudication_result.schema.json", &adjudication_path)
            .is_err()
    {
        reason_codes.insert(String::from("SCHEMA_INVALID"));
    }
    if patch_attestation_path.exists()
        && validate_json_file_against_schema("patch_attestation.schema.json", &patch_attestation_path)
            .is_err()
    {
        reason_codes.insert(String::from("SCHEMA_INVALID"));
    }
    if replay_record_path.exists()
        && validate_json_file_against_schema("replay_record.schema.json", &replay_record_path).is_err()
    {
        reason_codes.insert(String::from("SCHEMA_INVALID"));
    }

    let evidence_graph: Option<EvidenceGraph> = read_if_exists(&evidence_graph_path)?;
    let adjudication: Option<AdjudicationResult> = read_if_exists(&adjudication_path)?;
    let patch_attestation: Option<PatchAttestation> = read_if_exists(&patch_attestation_path)?;
    let replay_record: Option<ReplayRecord> = read_if_exists(&replay_record_path)?;

    if let (Some(evidence_graph), Some(adjudication), Some(attestation), Some(replay_record)) = (
        evidence_graph.as_ref(),
        adjudication.as_ref(),
        patch_attestation.as_ref(),
        replay_record.as_ref(),
    ) {
        if validate_cross_references(evidence_graph, adjudication, attestation, replay_record).is_err()
        {
            reason_codes.insert(String::from("BROKEN_REFERENCE"));
        }

        if validate_attestation_fields(attestation).is_err() {
            reason_codes.insert(String::from("ATTESTATION_FIELD_MISSING"));
        }

        if validate_artifact_refs(run_dir, evidence_graph, attestation, replay_record).is_err() {
            reason_codes.insert(String::from("BROKEN_REFERENCE"));
        }

        if validate_worktree_binding(run_dir, attestation).is_err() {
            reason_codes.insert(String::from("BROKEN_REFERENCE"));
        }

        if validate_attestation_policy(&replay_record.inputs.policy, attestation).is_err() {
            reason_codes.insert(String::from("BROKEN_REFERENCE"));
        }
    }

    let reason_codes: Vec<String> = reason_codes.into_iter().collect();
    let status = if reason_codes.is_empty() { "PASS" } else { "FAIL" };

    let report = VerifyReport {
        status: status.to_string(),
        reason_codes: reason_codes.clone(),
        checked_files,
    };

    let report_path = run_dir.join("verify_report.json");
    let body = serde_json::to_string_pretty(&report).context("failed to serialize verify report")?;
    fs::write(&report_path, body)
        .with_context(|| format!("failed to write {}", report_path.display()))?;

    if status == "PASS" {
        println!("PASS");
        Ok(report)
    } else {
        println!("FAIL {}", reason_codes.join(","));
        bail!(reason_codes.join(","))
    }
}

fn read_if_exists<T>(path: &Path) -> Result<Option<T>>
where
    T: serde::de::DeserializeOwned,
{
    if path.exists() {
        read_json_file(path).map(Some)
    } else {
        Ok(None)
    }
}

fn validate_cross_references(
    evidence_graph: &EvidenceGraph,
    adjudication: &AdjudicationResult,
    attestation: &PatchAttestation,
    replay_record: &ReplayRecord,
) -> Result<()> {
    if evidence_graph.run_id != adjudication.run_id || adjudication.run_id != replay_record.run_id {
        bail!("run ids do not align");
    }

    let proposal_ids: BTreeSet<&str> = evidence_graph
        .proposals
        .iter()
        .map(|proposal| proposal.proposal_id.as_str())
        .collect();

    if !proposal_ids.contains(adjudication.winning_proposal_id.as_str()) {
        bail!("winning proposal missing from evidence graph");
    }

    for critique in &evidence_graph.critiques {
        if !proposal_ids.contains(critique.proposal_id.as_str()) {
            bail!("critique references unknown proposal");
        }
    }

    if adjudication.final_writer != evidence_graph.final_writer {
        bail!("final writer mismatch");
    }

    if attestation.predicate.winning_proposal_id != adjudication.winning_proposal_id
        || attestation.predicate.final_writer != adjudication.final_writer
    {
        bail!("attestation does not match adjudication");
    }

    Ok(())
}

fn validate_attestation_fields(attestation: &PatchAttestation) -> Result<()> {
    if attestation.subject.task_id.is_empty()
        || attestation.subject.repository.is_empty()
        || attestation.builder.tool.is_empty()
        || attestation.predicate_type.is_empty()
        || attestation.policy.policy_id.is_empty()
        || attestation.verification_status.is_empty()
    {
        bail!("required attestation field missing");
    }

    Ok(())
}

fn validate_worktree_binding(run_dir: &Path, attestation: &PatchAttestation) -> Result<()> {
    let repository = resolve_run_path(run_dir, &attestation.subject.repository);
    if !repository.is_dir() {
        bail!("attested repository path does not exist as directory");
    }

    for rel in &attestation.subject.intended_files {
        let file = repository.join(rel);
        if !file.is_file() {
            bail!("intended file {} is missing from worktree", rel);
        }
    }

    Ok(())
}

fn validate_artifact_refs(
    run_dir: &Path,
    evidence_graph: &EvidenceGraph,
    attestation: &PatchAttestation,
    replay_record: &ReplayRecord,
) -> Result<()> {
    for artifact in evidence_graph
        .artifacts
        .iter()
        .chain(attestation.evidence_refs.iter())
        .chain(replay_record.generated_artifacts.iter())
    {
        let target = resolve_run_path(run_dir, &artifact.path);
        if !target.exists() {
            bail!("artifact {} does not exist", artifact.path);
        }

        if let Some(expected_sha) = artifact.sha256.as_deref() {
            let actual = sha256_for_file(&target)?;
            if actual != expected_sha {
                bail!("digest mismatch for {}", artifact.path);
            }
        }
    }

    Ok(())
}

fn validate_attestation_policy(policy_ref: &str, attestation: &PatchAttestation) -> Result<()> {
    let path = resolve_repo_path(policy_ref);
    let policy: Policy = read_json_file(&path)?;

    if policy.policy_id != attestation.policy.policy_id
        || policy.change_class != attestation.policy.change_class
        || policy.risk_level != attestation.policy.risk_level
    {
        bail!("attestation policy reference mismatch");
    }

    Ok(())
}

fn resolve_run_path(run_dir: &Path, reference: &str) -> PathBuf {
    let candidate = PathBuf::from(reference);
    if candidate.is_absolute() {
        candidate
    } else {
        run_dir.join(reference)
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::{
        AdjudicationResult, ArtifactRef, AttestationBuilder, AttestationSubject, ChecksRecord,
        Critique, EvidenceGraph, HumanWitness, PatchAttestation, PolicyRef, Predicate, Proposal,
        ReplayInputs, ReplayRecord, Task,
    };

    use super::validate_cross_references;

    #[test]
    fn cross_reference_validation_rejects_unknown_winner() {
        let evidence_graph = EvidenceGraph {
            run_id: String::from("run-1"),
            task: Task {
                task_id: String::from("task-1"),
                title: String::from("Task"),
                description: String::from("desc"),
                change_class: String::from("auth"),
                risk_level: String::from("high"),
                target_files: vec![String::from("src/a.ts")],
                requires_human_witness: true,
            },
            policy_ref: String::from("policy-1"),
            proposals: vec![Proposal {
                proposal_id: String::from("proposal-a"),
                proposer_role: String::from("security"),
                title: String::from("A"),
                summary: String::from("A"),
                final_writer_candidate: String::from("writer"),
                policy_compliance_score: 0.9,
                change_scope_score: 0.8,
                test_impact_score: 0.8,
                estimated_change_breadth: 2,
                predicted_files: vec![String::from("src/a.ts")],
            }],
            critiques: vec![Critique {
                critique_id: String::from("crit-1"),
                proposal_id: String::from("proposal-a"),
                reviewer_role: String::from("review"),
                verdict: String::from("support"),
                blocking: false,
                reason_code: String::from("NONE"),
                summary: String::from("ok"),
            }],
            adjudication_ref: String::from("adjudication.json"),
            artifacts: vec![ArtifactRef {
                path: String::from("adjudication.json"),
                sha256: None,
            }],
            final_writer: String::from("writer"),
        };

        let adjudication = AdjudicationResult {
            run_id: String::from("run-1"),
            winning_proposal_id: String::from("proposal-missing"),
            final_writer: String::from("writer"),
            ranking: vec![String::from("proposal-missing")],
            rejected_proposals: vec![],
            decision_basis: vec![String::from("basis")],
        };

        let attestation = PatchAttestation {
            subject: AttestationSubject {
                task_id: String::from("task-1"),
                title: String::from("Task"),
                repository: String::from("repo"),
                intended_files: vec![String::from("src/a.ts")],
            },
            builder: AttestationBuilder {
                tool: String::from("council"),
                version: String::from("0.1.0"),
            },
            predicate_type: String::from("predicate"),
            predicate: Predicate {
                winning_proposal_id: String::from("proposal-missing"),
                final_writer: String::from("writer"),
                review_mode: String::from("audit"),
                change_class: String::from("auth"),
                risk_level: String::from("high"),
            },
            policy: PolicyRef {
                policy_id: String::from("policy-1"),
                change_class: String::from("auth"),
                risk_level: String::from("high"),
            },
            checks: ChecksRecord {
                required: vec![],
                optional: vec![],
            },
            evidence_refs: vec![],
            human_witness: HumanWitness {
                required: true,
                status: String::from("pending"),
            },
            verification_status: String::from("pending"),
        };

        let replay_record = ReplayRecord {
            run_id: String::from("run-1"),
            inputs: ReplayInputs {
                task: String::from("fixtures/task.json"),
                policy: String::from("fixtures/policy.json"),
                proposals: String::from("fixtures/proposals.json"),
                critiques: String::from("fixtures/critiques.json"),
                checks: None,
            },
            generated_artifacts: vec![],
            verification_target: String::from("council verify --run out"),
        };

        assert!(validate_cross_references(
            &evidence_graph,
            &adjudication,
            &attestation,
            &replay_record
        )
        .is_err());
    }
}

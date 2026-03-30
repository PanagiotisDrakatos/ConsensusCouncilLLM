use std::{
    cmp::Ordering,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, ensure, Context, Result};

use crate::{
    policy::load_policy,
    schema::{
        read_json_file, repo_relative_or_absolute, sha256_for_path, validate_struct_against_schema,
        write_json_file, AdjudicationResult, ArtifactRef, ChecksRecord, Critique, EvidenceGraph,
        FixtureManifest, PatchAttestation, PolicyRef, Proposal, RejectedProposal, ReplayInputs,
        ReplayRecord, Task,
    },
};

const ADJUDICATION_SCHEMA: &str = "adjudication_result.schema.json";
const EVIDENCE_GRAPH_SCHEMA: &str = "evidence_graph.schema.json";
const FIXTURE_MANIFEST_SCHEMA: &str = "fixture_manifest.schema.json";
const PATCH_ATTESTATION_SCHEMA: &str = "patch_attestation.schema.json";
const REPLAY_RECORD_SCHEMA: &str = "replay_record.schema.json";

#[derive(Debug, Clone)]
struct RankedProposal {
    proposal: Proposal,
    blocking_reason: Option<String>,
}

#[derive(Debug, Clone)]
struct PatchBinding {
    worktree_dir: PathBuf,
    final_patch_diff: PathBuf,
}

pub fn execute_run(task: &Path, policy_path: &Path, out: &Path) -> Result<()> {
    let task_path = task.to_path_buf();
    let task: Task = read_json_file(&task_path)?;
    let policy = load_policy(policy_path)?;
    validate_task_policy_alignment(&task, &policy)?;

    let fixture_dir = task_path
        .parent()
        .map(Path::to_path_buf)
        .context("task path must include a parent directory")?;
    let fixture_manifest = load_fixture_manifest(&fixture_dir)?;
    let proposals_path = fixture_path(
        &fixture_dir,
        fixture_manifest
            .as_ref()
            .map(|manifest| manifest.proposals.as_str()),
        "proposals.json",
    );
    let critiques_path = fixture_path(
        &fixture_dir,
        fixture_manifest
            .as_ref()
            .map(|manifest| manifest.critiques.as_str()),
        "critiques.json",
    );
    let checks_path = fixture_manifest
        .as_ref()
        .and_then(|manifest| manifest.checks.as_deref())
        .map(|checks| fixture_dir.join(checks))
        .unwrap_or_else(|| fixture_dir.join("checks.json"));

    let proposals: Vec<Proposal> = read_json_file(&proposals_path)?;
    let critiques: Vec<Critique> = read_json_file(&critiques_path)?;
    let checks = if checks_path.exists() {
        read_json_file(&checks_path)?
    } else {
        ChecksRecord {
            required: Vec::new(),
            optional: Vec::new(),
        }
    };

    fs::create_dir_all(out).with_context(|| format!("failed to create {}", out.display()))?;

    let ranked = rank_proposals(&proposals, &critiques, policy.policy_compliance_threshold);
    let survivors: Vec<&RankedProposal> = ranked
        .iter()
        .filter(|entry| entry.blocking_reason.is_none())
        .collect();

    if survivors.is_empty() {
        bail!("no admissible proposal survived policy and critique checks");
    }

    let winner = &survivors[0].proposal;
    let run_id = format!("{}-{}", task.task_id, policy.policy_id);
    let adjudication = AdjudicationResult {
        run_id: run_id.clone(),
        winning_proposal_id: winner.proposal_id.clone(),
        final_writer: winner.final_writer_candidate.clone(),
        ranking: survivors
            .iter()
            .map(|entry| entry.proposal.proposal_id.clone())
            .collect(),
        rejected_proposals: ranked
            .iter()
            .filter_map(|entry| {
                entry.blocking_reason.as_ref().map(|reason| RejectedProposal {
                    proposal_id: entry.proposal.proposal_id.clone(),
                    reason: reason.clone(),
                })
            })
            .collect(),
        decision_basis: vec![
            format!(
                "Applied policy threshold {:.2} and blocked any proposal with a blocking critique.",
                policy.policy_compliance_threshold
            ),
            String::from(
                "Ranked surviving proposals by policy_compliance_score, change_scope_score, test_impact_score, lower estimated_change_breadth, then proposal_id.",
            ),
            format!(
                "Selected {} with final writer {}.",
                winner.proposal_id, winner.final_writer_candidate
            ),
        ],
    };
    validate_struct_against_schema(ADJUDICATION_SCHEMA, &adjudication)?;

    let adjudication_path = out.join("adjudication.json");
    write_json_file(&adjudication_path, &adjudication)?;

    let patch_binding = bind_replayed_patch_path(
        &fixture_dir,
        fixture_manifest.as_ref(),
        &winner.proposal_id,
        out,
    )?;

    let review_summary_path = out.join("review_summary.md");
    fs::write(
        &review_summary_path,
        render_review_summary(
            &task,
            &policy.policy_id,
            &adjudication,
            &ranked,
            &patch_binding,
            fixture_manifest
                .as_ref()
                .map(|manifest| manifest.fixture_id.as_str())
                .unwrap_or("fixture-defaults"),
        ),
    )
    .with_context(|| format!("failed to write {}", review_summary_path.display()))?;

    let replay_inputs = ReplayInputs {
        task: repo_relative_or_absolute(&task_path),
        policy: repo_relative_or_absolute(policy_path),
        proposals: repo_relative_or_absolute(&proposals_path),
        critiques: repo_relative_or_absolute(&critiques_path),
        checks: checks_path
            .exists()
            .then(|| repo_relative_or_absolute(&checks_path)),
    };

    let evidence_graph_path = out.join("evidence_graph.json");
    let replay_record_path = out.join("replay_record.json");
    let patch_attestation_path = out.join("patch_attestation.json");

    let replay_record = ReplayRecord {
        run_id: run_id.clone(),
        inputs: replay_inputs,
        generated_artifacts: Vec::new(),
        verification_target: String::from("council verify --run demo/sample_run"),
    };
    validate_struct_against_schema(REPLAY_RECORD_SCHEMA, &replay_record)?;
    write_json_file(&replay_record_path, &replay_record)?;

    let evidence_graph = EvidenceGraph {
        run_id: run_id.clone(),
        task: task.clone(),
        policy_ref: policy.policy_id.clone(),
        proposals: proposals.clone(),
        critiques: critiques.clone(),
        adjudication_ref: String::from("adjudication.json"),
        artifacts: vec![
            artifact_ref(&adjudication_path, true)?,
            artifact_ref(&patch_binding.final_patch_diff, true)?,
            artifact_ref(&patch_binding.worktree_dir, true)?,
            artifact_ref(&review_summary_path, true)?,
            artifact_ref(&replay_record_path, false)?,
        ],
        final_writer: adjudication.final_writer.clone(),
    };
    validate_struct_against_schema(EVIDENCE_GRAPH_SCHEMA, &evidence_graph)?;
    write_json_file(&evidence_graph_path, &evidence_graph)?;

    let attestation = PatchAttestation {
        subject: crate::schema::AttestationSubject {
            task_id: task.task_id.clone(),
            title: task.title.clone(),
            repository: String::from("worktree"),
            intended_files: winner.predicted_files.clone(),
        },
        builder: crate::schema::AttestationBuilder {
            tool: String::from("council"),
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
        predicate_type: String::from(
            "https://consensuscouncilllm.local/attestation/patch-adjudication/v1",
        ),
        predicate: crate::schema::Predicate {
            winning_proposal_id: adjudication.winning_proposal_id.clone(),
            final_writer: adjudication.final_writer.clone(),
            review_mode: policy.admission.consensus_mode.clone(),
            change_class: task.change_class.clone(),
            risk_level: task.risk_level.clone(),
        },
        policy: PolicyRef {
            policy_id: policy.policy_id.clone(),
            change_class: policy.change_class.clone(),
            risk_level: policy.risk_level.clone(),
        },
        checks,
        evidence_refs: vec![
            artifact_ref(&evidence_graph_path, false)?,
            artifact_ref(&adjudication_path, true)?,
            artifact_ref(&patch_binding.final_patch_diff, true)?,
            artifact_ref(&patch_binding.worktree_dir, true)?,
            artifact_ref(&review_summary_path, true)?,
            artifact_ref(&replay_record_path, false)?,
        ],
        human_witness: crate::schema::HumanWitness {
            required: policy.attestation.human_witness_required
                || policy.review.human_approval_required
                || task.requires_human_witness,
            status: if policy.attestation.human_witness_required
                || policy.review.human_approval_required
                || task.requires_human_witness
            {
                String::from("pending-human-approval")
            } else {
                String::from("not-required")
            },
        },
        verification_status: String::from("prototype-unverified-until-council-verify"),
    };
    validate_struct_against_schema(PATCH_ATTESTATION_SCHEMA, &attestation)?;
    write_json_file(&patch_attestation_path, &attestation)?;

    let replay_record = ReplayRecord {
        run_id: run_id.clone(),
        inputs: ReplayInputs {
            task: repo_relative_or_absolute(&task_path),
            policy: repo_relative_or_absolute(policy_path),
            proposals: repo_relative_or_absolute(&proposals_path),
            critiques: repo_relative_or_absolute(&critiques_path),
            checks: checks_path
                .exists()
                .then(|| repo_relative_or_absolute(&checks_path)),
        },
        generated_artifacts: vec![
            artifact_ref(&evidence_graph_path, false)?,
            artifact_ref(&adjudication_path, true)?,
            artifact_ref(&patch_attestation_path, false)?,
            artifact_ref(&patch_binding.final_patch_diff, true)?,
            artifact_ref(&patch_binding.worktree_dir, true)?,
            artifact_ref(&review_summary_path, true)?,
        ],
        verification_target: format!("council verify --run {}", out.display()),
    };
    validate_struct_against_schema(REPLAY_RECORD_SCHEMA, &replay_record)?;
    write_json_file(&replay_record_path, &replay_record)?;

    let evidence_graph = EvidenceGraph {
        run_id,
        task,
        policy_ref: policy.policy_id,
        proposals,
        critiques,
        adjudication_ref: String::from("adjudication.json"),
        artifacts: vec![
            artifact_ref(&adjudication_path, true)?,
            artifact_ref(&patch_attestation_path, false)?,
            artifact_ref(&patch_binding.final_patch_diff, true)?,
            artifact_ref(&patch_binding.worktree_dir, true)?,
            artifact_ref(&review_summary_path, true)?,
            artifact_ref(&replay_record_path, false)?,
        ],
        final_writer: adjudication.final_writer,
    };
    validate_struct_against_schema(EVIDENCE_GRAPH_SCHEMA, &evidence_graph)?;
    write_json_file(&evidence_graph_path, &evidence_graph)?;

    println!(
        "Generated governed change artifacts in {}",
        out.to_string_lossy()
    );

    Ok(())
}

fn load_fixture_manifest(fixture_dir: &Path) -> Result<Option<FixtureManifest>> {
    let manifest_path = fixture_dir.join("fixture_manifest.json");
    if !manifest_path.is_file() {
        return Ok(None);
    }

    let manifest: FixtureManifest = read_json_file(&manifest_path)?;
    validate_struct_against_schema(FIXTURE_MANIFEST_SCHEMA, &manifest)?;
    Ok(Some(manifest))
}

fn fixture_path(fixture_dir: &Path, configured: Option<&str>, default_name: &str) -> PathBuf {
    configured
        .map(|value| fixture_dir.join(value))
        .unwrap_or_else(|| fixture_dir.join(default_name))
}

fn bind_replayed_patch_path(
    fixture_dir: &Path,
    manifest: Option<&FixtureManifest>,
    winning_proposal_id: &str,
    out: &Path,
) -> Result<PatchBinding> {
    let baseline_repo = manifest
        .map(|manifest| fixture_dir.join(&manifest.baseline_repo))
        .unwrap_or_else(|| fixture_dir.join("repo"));
    let proposal_overlay = manifest
        .map(|manifest| {
            fixture_dir.join(render_template(
                &manifest.patch_overlay_dir,
                winning_proposal_id,
            ))
        })
        .unwrap_or_else(|| fixture_dir.join("patches").join(winning_proposal_id));
    let proposal_patch = manifest
        .map(|manifest| {
            fixture_dir.join(render_template(
                &manifest.patch_diff_template,
                winning_proposal_id,
            ))
        })
        .unwrap_or_else(|| {
            fixture_dir
                .join("patches")
                .join(format!("{winning_proposal_id}.patch"))
        });
    let worktree_dir = out.join("worktree");
    let final_patch_diff = out.join("final_patch.diff");

    ensure!(
        baseline_repo.is_dir(),
        "baseline fixture repo is missing at {}",
        baseline_repo.display()
    );
    ensure!(
        proposal_overlay.is_dir(),
        "proposal overlay is missing at {}",
        proposal_overlay.display()
    );
    ensure!(
        proposal_patch.is_file(),
        "proposal patch is missing at {}",
        proposal_patch.display()
    );

    if worktree_dir.exists() {
        fs::remove_dir_all(&worktree_dir)
            .with_context(|| format!("failed to clear {}", worktree_dir.display()))?;
    }

    copy_tree(&baseline_repo, &worktree_dir)?;
    copy_tree(&proposal_overlay, &worktree_dir)?;
    fs::copy(&proposal_patch, &final_patch_diff).with_context(|| {
        format!(
            "failed to copy replayed patch {} to {}",
            proposal_patch.display(),
            final_patch_diff.display()
        )
    })?;

    Ok(PatchBinding {
        worktree_dir,
        final_patch_diff,
    })
}

fn render_template(template: &str, winning_proposal_id: &str) -> String {
    template.replace("{proposal_id}", winning_proposal_id)
}

fn copy_tree(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)
        .with_context(|| format!("failed to create destination {}", dst.display()))?;

    for entry in fs::read_dir(src).with_context(|| format!("failed to read {}", src.display()))? {
        let entry = entry.with_context(|| format!("failed to read entry in {}", src.display()))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if entry
            .file_type()
            .with_context(|| format!("failed to inspect {}", src_path.display()))?
            .is_dir()
        {
            copy_tree(&src_path, &dst_path)?;
        } else {
            if let Some(parent) = dst_path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("failed to create parent directory {}", parent.display())
                })?;
            }
            fs::copy(&src_path, &dst_path).with_context(|| {
                format!(
                    "failed to copy {} to {}",
                    src_path.display(),
                    dst_path.display()
                )
            })?;
        }
    }

    Ok(())
}

fn validate_task_policy_alignment(task: &Task, policy: &crate::schema::Policy) -> Result<()> {
    ensure!(
        task.change_class == policy.change_class,
        "task change_class {} does not match policy {}",
        task.change_class,
        policy.change_class
    );
    ensure!(
        task.risk_level == policy.risk_level,
        "task risk_level {} does not match policy {}",
        task.risk_level,
        policy.risk_level
    );
    Ok(())
}

fn rank_proposals(
    proposals: &[Proposal],
    critiques: &[Critique],
    threshold: f64,
) -> Vec<RankedProposal> {
    let mut ranked: Vec<RankedProposal> = proposals
        .iter()
        .cloned()
        .map(|proposal| {
            let blocking_reason = collect_rejection_reason(&proposal, critiques, threshold);
            RankedProposal {
                proposal,
                blocking_reason,
            }
        })
        .collect();

    ranked.sort_by(compare_ranked_proposals);
    ranked
}

fn collect_rejection_reason(
    proposal: &Proposal,
    critiques: &[Critique],
    threshold: f64,
) -> Option<String> {
    if let Some(blocking_reason) = critiques
        .iter()
        .find(|critique| critique.proposal_id == proposal.proposal_id && critique.blocking)
        .map(|critique| critique.reason_code.clone())
    {
        return Some(blocking_reason);
    }

    if proposal.policy_compliance_score < threshold {
        return Some(format!(
            "POLICY_THRESHOLD_UNMET:{:.2}<{:.2}",
            proposal.policy_compliance_score, threshold
        ));
    }

    None
}

fn compare_ranked_proposals(left: &RankedProposal, right: &RankedProposal) -> Ordering {
    match (&left.blocking_reason, &right.blocking_reason) {
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        _ => compare_survivor_scores(&left.proposal, &right.proposal),
    }
}

fn compare_survivor_scores(left: &Proposal, right: &Proposal) -> Ordering {
    compare_desc(left.policy_compliance_score, right.policy_compliance_score)
        .then_with(|| compare_desc(left.change_scope_score, right.change_scope_score))
        .then_with(|| compare_desc(left.test_impact_score, right.test_impact_score))
        .then_with(|| {
            left.estimated_change_breadth
                .cmp(&right.estimated_change_breadth)
        })
        .then_with(|| left.proposal_id.cmp(&right.proposal_id))
}

fn compare_desc(left: f64, right: f64) -> Ordering {
    right.partial_cmp(&left).unwrap_or(Ordering::Equal)
}

fn artifact_ref(path: &Path, include_sha: bool) -> Result<ArtifactRef> {
    Ok(ArtifactRef {
        path: path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string()),
        sha256: if include_sha {
            Some(sha256_for_path(path)?)
        } else {
            None
        },
    })
}

fn render_review_summary(
    task: &Task,
    policy_id: &str,
    adjudication: &AdjudicationResult,
    ranked: &[RankedProposal],
    patch_binding: &PatchBinding,
    fixture_id: &str,
) -> String {
    let rejected = ranked
        .iter()
        .filter_map(|entry| {
            entry
                .blocking_reason
                .as_ref()
                .map(|reason| format!("- `{}` rejected: {}", entry.proposal.proposal_id, reason))
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "# Governed Change Review Summary\n\n\
## Task\n\
- Task: `{}`\n\
- Title: {}\n\
- Policy: `{}`\n\
- Fixture: `{}`\n\
- Outcome: replayed patch candidate applied to fixed local worktree\n\n\
## Decision\n\
- Winning proposal: `{}`\n\
- Final writer: `{}`\n\
- Ranked survivors: {}\n\n\
## Rejections\n\
{}\n\n\
## Patch path\n\
- Applied worktree: `{}`\n\
- Patch diff: `{}`\n\
\n\
## Patch metadata\n\
- Intended files: {}\n\
- Human witness: pending\n\
- Artifact set: `evidence_graph.json`, `adjudication.json`, `patch_attestation.json`, `review_summary.md`, `replay_record.json`, `final_patch.diff`, `worktree/`\n",
        task.task_id,
        task.title,
        policy_id,
        fixture_id,
        adjudication.winning_proposal_id,
        adjudication.final_writer,
        adjudication.ranking.join(", "),
        if rejected.is_empty() {
            String::from("- none")
        } else {
            rejected
        },
        patch_binding
            .worktree_dir
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| patch_binding.worktree_dir.to_string_lossy().into_owned()),
        patch_binding
            .final_patch_diff
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| patch_binding.final_patch_diff.to_string_lossy().into_owned()),
        task.target_files.join(", "),
    )
}

#[cfg(test)]
mod tests {
    use crate::schema::{Critique, Proposal};

    use super::rank_proposals;

    #[test]
    fn adjudication_prefers_best_compliant_nonblocked_proposal() {
        let proposals = vec![
            Proposal {
                proposal_id: String::from("a"),
                proposer_role: String::from("arch"),
                title: String::from("A"),
                summary: String::from("A"),
                final_writer_candidate: String::from("writer-a"),
                policy_compliance_score: 0.90,
                change_scope_score: 0.85,
                test_impact_score: 0.80,
                estimated_change_breadth: 2,
                predicted_files: vec![String::from("a.rs")],
            },
            Proposal {
                proposal_id: String::from("b"),
                proposer_role: String::from("security"),
                title: String::from("B"),
                summary: String::from("B"),
                final_writer_candidate: String::from("writer-b"),
                policy_compliance_score: 0.95,
                change_scope_score: 0.87,
                test_impact_score: 0.90,
                estimated_change_breadth: 2,
                predicted_files: vec![String::from("b.rs")],
            },
            Proposal {
                proposal_id: String::from("c"),
                proposer_role: String::from("wide"),
                title: String::from("C"),
                summary: String::from("C"),
                final_writer_candidate: String::from("writer-c"),
                policy_compliance_score: 0.82,
                change_scope_score: 0.60,
                test_impact_score: 0.60,
                estimated_change_breadth: 6,
                predicted_files: vec![String::from("c.rs")],
            },
        ];
        let critiques = vec![Critique {
            critique_id: String::from("crit-c"),
            proposal_id: String::from("c"),
            reviewer_role: String::from("architecture"),
            verdict: String::from("block"),
            blocking: true,
            reason_code: String::from("SCOPE_TOO_BROAD"),
            summary: String::from("too broad"),
        }];

        let ranked = rank_proposals(&proposals, &critiques, 0.80);
        assert_eq!(ranked[0].proposal.proposal_id, "b");
        assert_eq!(ranked[0].blocking_reason, None);
        assert_eq!(
            ranked[2].blocking_reason.as_deref(),
            Some("SCOPE_TOO_BROAD")
        );
    }
}

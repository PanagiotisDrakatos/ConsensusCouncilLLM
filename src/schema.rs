use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use jsonschema::validator_for;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub task_id: String,
    pub title: String,
    pub description: String,
    pub change_class: String,
    pub risk_level: String,
    pub target_files: Vec<String>,
    pub requires_human_witness: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub policy_id: String,
    pub change_class: String,
    pub risk_level: String,
    pub admission: Admission,
    pub review: ReviewRequirements,
    pub required_artifacts: Vec<String>,
    pub checks: CheckPolicy,
    pub merge_gate: MergeGate,
    pub attestation: AttestationPolicy,
    pub policy_compliance_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admission {
    pub consensus_required: bool,
    pub consensus_mode: String,
    pub allow_direct_execution: bool,
    pub require_final_writer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewRequirements {
    pub minimum_reviewers: u64,
    pub reviewer_roles: Vec<String>,
    pub human_approval_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPolicy {
    pub required: Vec<String>,
    pub optional: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeGate {
    pub allow_merge_if: Vec<String>,
    pub block_merge_if: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationPolicy {
    pub level: String,
    pub export_formats: Vec<String>,
    pub human_witness_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub proposal_id: String,
    pub proposer_role: String,
    pub title: String,
    pub summary: String,
    pub final_writer_candidate: String,
    pub policy_compliance_score: f64,
    pub change_scope_score: f64,
    pub test_impact_score: f64,
    pub estimated_change_breadth: u64,
    pub predicted_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Critique {
    pub critique_id: String,
    pub proposal_id: String,
    pub reviewer_role: String,
    pub verdict: String,
    pub blocking: bool,
    pub reason_code: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckRun {
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecksRecord {
    pub required: Vec<CheckRun>,
    pub optional: Vec<CheckRun>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRef {
    pub path: String,
    pub sha256: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceGraph {
    pub run_id: String,
    pub task: Task,
    pub policy_ref: String,
    pub proposals: Vec<Proposal>,
    pub critiques: Vec<Critique>,
    pub adjudication_ref: String,
    pub artifacts: Vec<ArtifactRef>,
    pub final_writer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectedProposal {
    pub proposal_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjudicationResult {
    pub run_id: String,
    pub winning_proposal_id: String,
    pub final_writer: String,
    pub ranking: Vec<String>,
    pub rejected_proposals: Vec<RejectedProposal>,
    pub decision_basis: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationSubject {
    pub task_id: String,
    pub title: String,
    pub repository: String,
    pub intended_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationBuilder {
    pub tool: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Predicate {
    pub winning_proposal_id: String,
    pub final_writer: String,
    pub review_mode: String,
    pub change_class: String,
    pub risk_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanWitness {
    pub required: bool,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAttestation {
    pub subject: AttestationSubject,
    pub builder: AttestationBuilder,
    pub predicate_type: String,
    pub predicate: Predicate,
    pub policy: PolicyRef,
    pub checks: ChecksRecord,
    pub evidence_refs: Vec<ArtifactRef>,
    pub human_witness: HumanWitness,
    pub verification_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRef {
    pub policy_id: String,
    pub change_class: String,
    pub risk_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayRecord {
    pub run_id: String,
    pub inputs: ReplayInputs,
    pub generated_artifacts: Vec<ArtifactRef>,
    pub verification_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayInputs {
    pub task: String,
    pub policy: String,
    pub proposals: String,
    pub critiques: String,
    pub checks: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureManifest {
    pub fixture_id: String,
    pub proposals: String,
    pub critiques: String,
    pub checks: Option<String>,
    pub baseline_repo: String,
    pub patch_overlay_dir: String,
    pub patch_diff_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyReport {
    pub status: String,
    pub reason_codes: Vec<String>,
    pub checked_files: Vec<String>,
}

pub fn manifest_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn schema_dir() -> PathBuf {
    manifest_root().join("schemas")
}

pub fn schema_path(schema_name: &str) -> PathBuf {
    schema_dir().join(schema_name)
}

pub fn read_json_file<T>(path: &Path) -> Result<T>
where
    T: DeserializeOwned,
{
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read JSON file {}", path.display()))?;
    serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse JSON file {}", path.display()))
}

pub fn write_json_file<T>(path: &Path, value: &T) -> Result<()>
where
    T: Serialize,
{
    let body = serde_json::to_string_pretty(value)
        .with_context(|| format!("failed to serialize JSON for {}", path.display()))?;
    fs::write(path, body).with_context(|| format!("failed to write JSON file {}", path.display()))
}

pub fn serialize_to_value<T>(value: &T) -> Result<Value>
where
    T: Serialize,
{
    serde_json::to_value(value).context("failed to convert structure to JSON value")
}

pub fn validate_value_against_schema(schema_name: &str, value: &Value) -> Result<()> {
    let schema_path = schema_path(schema_name);
    let schema: Value = read_json_file(&schema_path)?;
    let validator = validator_for(&schema)
        .with_context(|| format!("failed to compile {}", schema_path.display()))?;

    let errors: Vec<String> = validator
        .iter_errors(value)
        .map(|err| err.to_string())
        .collect();
    if errors.is_empty() {
        Ok(())
    } else {
        bail!(
            "schema validation failed for {}: {}",
            schema_name,
            errors.join("; ")
        )
    }
}

pub fn validate_struct_against_schema<T>(schema_name: &str, value: &T) -> Result<()>
where
    T: Serialize,
{
    let json_value = serialize_to_value(value)?;
    validate_value_against_schema(schema_name, &json_value)
}

pub fn validate_json_file_against_schema(schema_name: &str, path: &Path) -> Result<()> {
    let instance: Value = read_json_file(path)?;
    validate_value_against_schema(schema_name, &instance)
}

pub fn sha256_for_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

pub fn sha256_for_file(path: &Path) -> Result<String> {
    let bytes = fs::read(path)
        .with_context(|| format!("failed to read file for digest {}", path.display()))?;
    Ok(sha256_for_bytes(&bytes))
}

pub fn sha256_for_path(path: &Path) -> Result<String> {
    if path.is_file() {
        return sha256_for_file(path);
    }

    if path.is_dir() {
        return sha256_for_directory(path);
    }

    bail!("cannot compute digest for missing path {}", path.display())
}

fn sha256_for_directory(path: &Path) -> Result<String> {
    let mut files = Vec::new();
    collect_directory_files(path, path, &mut files)?;
    files.sort();

    let mut manifest = Vec::new();
    for rel in files {
        let rel_display = rel.to_string_lossy().replace('\\', "/");
        let digest = sha256_for_file(&path.join(&rel))?;
        manifest.extend_from_slice(rel_display.as_bytes());
        manifest.push(0);
        manifest.extend_from_slice(digest.as_bytes());
        manifest.push(b'\n');
    }

    Ok(sha256_for_bytes(&manifest))
}

fn collect_directory_files(root: &Path, current: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    let mut entries = fs::read_dir(current)
        .with_context(|| format!("failed to read directory {}", current.display()))?
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| format!("failed to enumerate directory {}", current.display()))?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();
        if entry
            .file_type()
            .with_context(|| format!("failed to inspect {}", path.display()))?
            .is_dir()
        {
            collect_directory_files(root, &path, files)?;
        } else {
            files.push(
                path.strip_prefix(root)
                    .with_context(|| format!("failed to relativize {}", path.display()))?
                    .to_path_buf(),
            );
        }
    }

    Ok(())
}

pub fn repo_relative_or_absolute(path: &Path) -> String {
    let manifest_root = manifest_root();
    if let Ok(relative) = path.strip_prefix(&manifest_root) {
        relative.to_string_lossy().replace('\\', "/")
    } else {
        path.to_string_lossy().replace('\\', "/")
    }
}

pub fn resolve_repo_path(path_str: &str) -> PathBuf {
    let path = PathBuf::from(path_str);
    if path.is_absolute() {
        path
    } else {
        manifest_root().join(path)
    }
}

use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::schema::{validate_struct_against_schema, Policy};

pub fn load_policy(path: &Path) -> Result<Policy> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read policy file {}", path.display()))?;
    let policy = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse policy file {}", path.display()))?;
    validate_struct_against_schema("policy.schema.json", &policy)?;
    Ok(policy)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::load_policy;

    #[test]
    fn loads_fixture_policy_and_matches_schema() {
        let path = Path::new("fixtures/auth_validation_fix/policy.json");
        let policy = load_policy(path).expect("load policy");
        assert_eq!(policy.policy_id, "auth_security_v1");
        assert_eq!(policy.required_artifacts.len(), 7);
    }
}

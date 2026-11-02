use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InvariantAuditResult {
    pub invariant_code: String,
    pub description: String,
    pub passed: bool,
    pub nominal_test_passed: bool,
    pub adversarial_rejection_passed: bool,
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestExecutionReport {
    pub suite_name: String,
    pub stage: String,
    pub timestamp: u64,
    pub commit_ref: String,
    pub total_invariants_tested: usize,
    pub total_passed: usize,
    pub all_invariants_satisfied: bool,
    pub report_content_digest: String,
    pub results: Vec<InvariantAuditResult>,
}

impl TestExecutionReport {
    pub fn new(stage: &str, commit_ref: &str, results: Vec<InvariantAuditResult>) -> Self {
        let total_invariants_tested = results.len();
        let total_passed = results.iter().filter(|r| r.passed).count();
        let all_invariants_satisfied = total_passed == total_invariants_tested;
        let timestamp = chrono::Utc::now().timestamp() as u64;

        let mut report = Self {
            suite_name: "Arkhe-Test Continuous Invariant Pipeline".to_string(),
            stage: stage.to_string(),
            timestamp,
            commit_ref: commit_ref.to_string(),
            total_invariants_tested,
            total_passed,
            all_invariants_satisfied,
            report_content_digest: String::new(),
            results,
        };

        let json = serde_json::to_string(&report.results).unwrap_or_default();
        let digest = crate::domain::hash_sha256(json.as_bytes());
        report.report_content_digest = format!("bafkreport{}", hex::encode(digest));
        report
    }
}

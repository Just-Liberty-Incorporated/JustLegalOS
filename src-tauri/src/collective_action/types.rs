use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoPlaintiffEntry {
    pub name: String,
    pub city_state: String,
    pub injury_summary: String,
    pub date_of_injury_or_hold: String,
    pub evidence_file: Option<String>,
    pub sha256_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CollectiveActionOutput {
    pub mode: String, // "rule20-joinder", "rule23-binder", "rulemaking-petition"
    pub title: String,
    pub statutory_authority: String,
    pub why_this_works_pro_se: String,
    pub total_plaintiffs_or_claimants: usize,
    pub full_rendered_text: String,
    pub output_file_path: Option<String>,
}

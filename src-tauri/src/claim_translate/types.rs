use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClaimTranslation {
    pub raw_query: String,
    pub primary_match: String,
    pub why_vernacular_fails_in_court: String,
    pub legal_causes_of_action: Vec<LegalCauseOfAction>,
    pub mountain_of_proof: MountainOfProof,
    pub legal_pivots: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LegalCauseOfAction {
    pub title: String,
    pub statutory_citation: String,
    pub target_entity_type: String, // "Private Contractor", "Official Capacity", "Individual Capacity"
    pub relief_available: String,   // "Treble Damages & Bounty", "Prospective Injunction & Declaratory", "Damages (Abrogated Immunity)"
    pub governing_standard: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MountainOfProof {
    pub element_x_standing: String,
    pub element_y_enterprise_or_hook: String,
    pub element_z_mountain_of_evidence: Vec<String>,
    pub required_records_requests: Vec<String>,
}

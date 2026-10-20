use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SovereignImmunityReport {
    pub state: String,
    pub target_type: String, // "gov-agency", "gov-contractor", "official-individual", "local-gov"
    pub general_rule: String,
    pub retrospective_damages_barred: bool,
    pub active_bypass_pathways: Vec<ImmunityBypassPathway>,
    pub claims_evaluated: Vec<ClaimImmunityResult>,
    pub overall_recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImmunityBypassPathway {
    pub doctrine_name: String,
    pub citation: String,
    pub relief_type: String, // "Prospective Injunction & Declaratory", "Money Damages & Injunction", "Full Treble Damages"
    pub applicable_target: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClaimImmunityResult {
    pub claim_name: String,
    pub viable: bool,
    pub legal_basis: String,
    pub recommended_defendant: String,
}

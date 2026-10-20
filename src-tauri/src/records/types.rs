use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecordsLaw {
    pub name: String,
    pub citation: String,
    pub jurisdiction: String,
    pub response_working_days: u32,
    pub extension_working_days: u32,
    pub fee_structure: String,
    pub exemptions_of_note: Vec<String>,
    pub custodians: Vec<RecordsCustodian>,
    pub own_file_route: Option<String>,
    pub source: String,
    pub verified_on: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecordsCustodian {
    pub agency: String,
    pub email: Option<String>,
    pub portal_url: Option<String>,
    pub mailing_address: Option<String>,
    pub fee_rate: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrgProfile {
    pub name: String,
    pub ein: String,
    pub legal_structure: String,
    pub sos_id: String,
    pub address: String,
    pub email: String,
    pub phone: String,
    pub fee_waiver_statement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecordsDraftRequest {
    pub request_id: String,
    pub jurisdiction: String,
    pub record_type: String,
    pub custodian_agency: String,
    pub custodian_email: Option<String>,
    pub subject: String,
    pub items_requested: Vec<String>,
    pub formal_letter_text: String,
    pub fee_waiver_requested: bool,
    pub fee_waiver_text: String,
    pub estimated_cost_notice: String,
    pub mailing_instructions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrackedRequestEntry {
    pub id: String,
    pub law_name: String,
    pub agency: String,
    pub sent_date: String,
    pub due_date: String,
    pub extension_date: Option<String>,
    pub tracking_number: Option<String>,
    pub payment_reference: Option<String>,
    pub cost_dollars: Option<f64>,
    pub status: String, // "Sent", "Extended", "Responded", "Denied", "Appealed"
    pub response_text_summary: Option<String>,
    pub next_step: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecordsTrackerManifest {
    pub case_id: String,
    pub requests: Vec<TrackedRequestEntry>,
    pub last_updated: String,
}

use super::types::OrgProfile;

pub fn just_liberty_profile() -> OrgProfile {
    OrgProfile {
        name: "Just Liberty Incorporated".to_string(),
        ein: "93-3704483".to_string(),
        legal_structure: "501(c)(3) Nonprofit Public Benefit Organization (509(a)(2))".to_string(),
        sos_id: "20238046909".to_string(),
        address: "PO Box 761, Loveland, CO 80539".to_string(),
        email: "legal@justliberty.org".to_string(),
        phone: "(970) 555-0199".to_string(),
        fee_waiver_statement: "Just Liberty Inc. is a 501(c)(3) public interest research and civil liberties organization requesting these records strictly for non-commercial research, public accountability, and journalistic analysis under applicable statutory fee waiver provisions.".to_string(),
    }
}

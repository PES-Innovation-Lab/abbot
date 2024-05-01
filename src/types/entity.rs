use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub srn: String,
    pub name: String,
    pub resume: String,
    pub email: String,
    pub contact: String,
    pub year_of_study: String,
    pub branch: String,
    pub campus: String,
    pub gender: String,
    pub interview_slot_id: String,
    pub panel_id: String,
    pub ranked_domains: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    pub name: String
}
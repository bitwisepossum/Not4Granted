use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum GrantStatus {
    Planning,
    Submitted,
    Accepted,
    Rejected,
}

impl GrantStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Planning => "Planning",
            Self::Submitted => "Submitted",
            Self::Accepted => "Accepted",
            Self::Rejected => "Rejected",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ManuscriptStatus {
    Idea,
    Drafting,
    Submitted,
    Revision,
    Accepted,
    Published,
    Rejected,
}

impl ManuscriptStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idea => "Idea",
            Self::Drafting => "Drafting",
            Self::Submitted => "Submitted",
            Self::Revision => "Revision",
            Self::Accepted => "Accepted",
            Self::Published => "Published",
            Self::Rejected => "Rejected",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NewGrant {
    pub name: String,
    pub funder: String,
    pub call_name: Option<String>,
    pub status: GrantStatus,
    pub amount_requested: Option<i64>,
    pub amount_received: Option<i64>,
    pub currency: String,
    pub deadline: Option<String>,
    pub submitted_at: Option<String>,
    pub decision_at: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grant {
    pub id: i64,
    pub name: String,
    pub funder: String,
    pub call_name: Option<String>,
    pub status: GrantStatus,
    pub amount_requested: Option<i64>,
    pub amount_received: Option<i64>,
    pub currency: String,
    pub deadline: Option<String>,
    pub submitted_at: Option<String>,
    pub decision_at: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NewManuscript {
    pub title: String,
    pub short_name: Option<String>,
    pub journal: Option<String>,
    pub status: ManuscriptStatus,
    pub next_action: Option<String>,
    pub submitted_at: Option<String>,
    pub decision_at: Option<String>,
    pub published_at: Option<String>,
    pub doi: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manuscript {
    pub id: i64,
    pub title: String,
    pub short_name: Option<String>,
    pub journal: Option<String>,
    pub status: ManuscriptStatus,
    pub next_action: Option<String>,
    pub submitted_at: Option<String>,
    pub decision_at: Option<String>,
    pub published_at: Option<String>,
    pub doi: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct GrantQuery {
    pub statuses: Option<Vec<GrantStatus>>,
    pub funders: Option<Vec<String>>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub direction: Option<String>,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum GrantSort {
    #[default]
    Deadline,
    Name,
    Status,
    Funder,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}
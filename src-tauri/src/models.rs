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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct GrantQuery {
    pub id: Option<i64>,
    pub statuses: Vec<GrantStatus>,
    pub funders: Vec<String>,
    pub search: Option<String>,
    pub sort_by: GrantSort,
    pub direction: SortDirection,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum GrantSort {
    #[default]
    Deadline,
    Name,
    Status,
    Funder,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

impl TryFrom<String> for GrantStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Planning" => Ok(Self::Planning),
            "Submitted" => Ok(Self::Submitted),
            "Accepted" => Ok(Self::Accepted),
            "Rejected" => Ok(Self::Rejected),
            _ => Err(format!("Invalid grant status: {}", value)),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct ManuscriptQuery {
    pub id: Option<i64>,
    pub statuses: Vec<ManuscriptStatus>,
    pub search: Option<String>,
    pub sort_by: ManuscriptSort,
    pub direction: SortDirection,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum ManuscriptSort {
    #[default]
    Title,
    Status,
    Journal,
}

impl TryFrom<String> for ManuscriptStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Idea" => Ok(Self::Idea),
            "Drafting" => Ok(Self::Drafting),
            "Submitted" => Ok(Self::Submitted),
            "Revision" => Ok(Self::Revision),
            "Accepted" => Ok(Self::Accepted),
            "Published" => Ok(Self::Published),
            "Rejected" => Ok(Self::Rejected),
            _ => Err(format!("Invalid manuscript status: {}", value)),
        }
    }
}
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

const DB_FILE: &str = "db.sqlite3";

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

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
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

pub fn open_database() -> Result<Connection> {
    let conn = Connection::open(DB_FILE)?;
    
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    let version: i32 =
        conn.query_row("PRAGMA user_version;", [], |row| row.get(0))?;

    match version {
        0 => {
            create_schema(&conn)?;
        }

        1 => {
            // Database is already at the correct version
        }

        _ => {
            panic!("Unsupported database version: {}", version);
        }
    }

    Ok(conn)
}

pub fn add_grant_to_database(new_grant: NewGrant) -> Result<Grant> {
    let conn = open_database()?;

    conn.execute(
        r#"
        INSERT INTO grant (
            name,
            funder,
            call_name,
            status,
            amount_requested,
            amount_received,
            currency,
            deadline,
            submitted_at,
            decision_at,
            notes
        )
        VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6,
            ?7, ?8, ?9, ?10, ?11
        )
        "#,
        params![
            &new_grant.name,
            &new_grant.funder,
            &new_grant.call_name,
            new_grant.status.as_str(),
            new_grant.amount_requested,
            new_grant.amount_received,
            &new_grant.currency,
            &new_grant.deadline,
            &new_grant.submitted_at,
            &new_grant.decision_at,
            &new_grant.notes,
        ],
    )?;

    let id = conn.last_insert_rowid();

    Ok(Grant {
        id,
        name: new_grant.name,
        funder: new_grant.funder,
        call_name: new_grant.call_name,
        status: new_grant.status,
        amount_requested: new_grant.amount_requested,
        amount_received: new_grant.amount_received,
        currency: new_grant.currency,
        deadline: new_grant.deadline,
        submitted_at: new_grant.submitted_at,
        decision_at: new_grant.decision_at,
        notes: new_grant.notes,
    })
}

fn create_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        PRAGMA foreign_keys = ON;

        CREATE TABLE grant (
            id INTEGER PRIMARY KEY AUTOINCREMENT,

            name TEXT NOT NULL,
            funder TEXT NOT NULL,
            call_name TEXT,

            status TEXT NOT NULL DEFAULT 'Planning'
                CHECK (
                    status IN (
                        'Planning',
                        'Submitted',
                        'Accepted',
                        'Rejected'
                    )
                ),

            amount_requested INTEGER,
            amount_received INTEGER,
            currency TEXT NOT NULL DEFAULT 'EUR',

            deadline TEXT,
            submitted_at TEXT,
            decision_at TEXT,

            notes TEXT,

            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE manuscript (
            id INTEGER PRIMARY KEY AUTOINCREMENT,

            title TEXT NOT NULL,
            short_name TEXT,
            journal TEXT,

            status TEXT NOT NULL DEFAULT 'Idea'
                CHECK (
                    status IN (
                        'Idea',
                        'Drafting',
                        'Submitted',
                        'Revision',
                        'Accepted',
                        'Published',
                        'Rejected'
                    )
                ),

            next_action TEXT,

            submitted_at TEXT,
            decision_at TEXT,
            published_at TEXT,

            doi TEXT,
            notes TEXT,

            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE grant_manuscript (
            grant_id INTEGER NOT NULL,
            manuscript_id INTEGER NOT NULL,

            PRIMARY KEY (grant_id, manuscript_id),

            FOREIGN KEY (grant_id)
                REFERENCES grant(id)
                ON DELETE CASCADE,

            FOREIGN KEY (manuscript_id)
                REFERENCES manuscript(id)
                ON DELETE CASCADE
        );

        CREATE INDEX idx_grant_status
            ON grant(status);

        CREATE INDEX idx_grant_deadline
            ON grant(deadline);

        CREATE INDEX idx_manuscript_status
            ON manuscript(status);

        PRAGMA user_version = 1;
        "#,
    )?;

    Ok(())
}
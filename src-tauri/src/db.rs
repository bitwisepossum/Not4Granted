use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

const DB_FILE: &str = "db.sqlite3";

#[derive(Debug, Deserialize)]
pub struct NewGrant {
    pub name: String,
    pub funder: String,
    pub deadline: String,
    pub amount: String,
    pub status: GrantStatus,
}

#[derive(Debug, Serialize)]
pub struct Grant {
    pub id: i64,
    pub name: String,
    pub funder: String,
    pub deadline: String,
    pub amount: String,
    pub status: GrantStatus,
}

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

#[derive(Debug)]
struct Manuscript {
    id: i32,
    title: String,
    short_name: String,
    journal: String,
    status: ManuscriptStatus,
}

#[derive(Debug)]
enum ManuscriptStatus {
    Draft,
    Submitted,
    Accepted,
    Rejected,
    Published,
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
        "INSERT INTO grant (name, funder, deadline, amount_requested, status) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            new_grant.name,
            new_grant.funder,
            new_grant.deadline,
            new_grant.amount,
            format!("{:?}", new_grant.status),
        ],
    )?;

    let id = conn.last_insert_rowid();

    Ok(Grant {
        id,
        name: new_grant.name,
        funder: new_grant.funder,
        deadline: new_grant.deadline,
        amount: new_grant.amount,
        status: new_grant.status,
    })
}

fn create_schema (conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"

        CREATE TABLE grant (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            funder TEXT,
            call_name TEXT,
            status TEXT NOT NULL DEFAULT 'draft',
            amount_requested INTEGER,
            amount_received INTEGER,
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
            status TEXT NOT NULL DEFAULT 'draft',
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

        CREATE INDEX IF NOT EXISTS idx_grant_status
            ON grant(status);

        CREATE INDEX IF NOT EXISTS idx_grant_deadline
            ON grant(deadline);

        CREATE INDEX IF NOT EXISTS idx_manuscript_status
            ON manuscript(status);

        PRAGMA user_version = 1;
        "#,
    )?;

    Ok(())
}
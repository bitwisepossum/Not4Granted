use rusqlite::{params, Connection, Result};
use crate::models::{GrantStatus, ManuscriptStatus, NewGrant, Grant, NewManuscript, Manuscript};

const DB_FILE: &str = "db.sqlite3";

/**
 * General database functions
 * open_database: Opens the database and creates the schema if it doesn't exist
 * create_schema: Creates the database schema
 */
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

/**
 * Grant functions
 * add_grant_to_database: Adds a new grant to the database
 * get_grants_from_database: Retrieves all grants from the database
 * get_grant_by_id_from_database: Retrieves a grant by its ID from the database
 */
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

pub fn get_grants_from_database() -> Result<Vec<Grant>> {
    let conn = open_database()?;

    let mut stmt = conn.prepare(
        r#"
        SELECT
            id,
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
        FROM grant
        "#,
    )?;

    // Map the rows to Grant structs
    // Use query_map to iterate over the rows and map them to Grant structs
    let grants_iter = stmt.query_map([], |row| {
        Ok(Grant {
            id: row.get(0)?,
            name: row.get(1)?,
            funder: row.get(2)?,
            call_name: row.get(3)?,
            status: match row.get::<_, String>(4)?.as_str() {
                "Planning" => GrantStatus::Planning,
                "Submitted" => GrantStatus::Submitted,
                "Accepted" => GrantStatus::Accepted,
                "Rejected" => GrantStatus::Rejected,
                _ => panic!("Invalid grant status"),
            },
            amount_requested: row.get(5)?,
            amount_received: row.get(6)?,
            currency: row.get(7)?,
            deadline: row.get(8)?,
            submitted_at: row.get(9)?,
            decision_at: row.get(10)?,
            notes: row.get(11)?,
        })
    })?;

    let grants: Vec<Grant> = grants_iter.collect::<Result<Vec<_>, _>>()?;

    Ok(grants)
}

pub fn get_grant_by_id_from_database(grant_id: i64) -> Result<Grant> {
    let conn = open_database()?;

    let mut stmt = conn.prepare(
        r#"
        SELECT
            id,
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
        FROM grant
        WHERE id = ?1
        "#,
    )?;

    let grant_row = stmt.query_row(params![grant_id], |row| {
        Ok(Grant {
            id: row.get(0)?,
            name: row.get(1)?,
            funder: row.get(2)?,
            call_name: row.get(3)?,
            status: match row.get::<_, String>(4)?.as_str() {
                "Planning" => GrantStatus::Planning,
                "Submitted" => GrantStatus::Submitted,
                "Accepted" => GrantStatus::Accepted,
                "Rejected" => GrantStatus::Rejected,
                _ => panic!("Invalid grant status"),
            },
            amount_requested: row.get(5)?,
            amount_received: row.get(6)?,
            currency: row.get(7)?,
            deadline: row.get(8)?,
            submitted_at: row.get(9)?,
            decision_at: row.get(10)?,
            notes: row.get(11)?,
        })
    })?;

    Ok(grant_row)
}

pub fn update_grant_in_database(grant: Grant) -> Result<()> {
    let conn = open_database()?;

    conn.execute(
        r#"
        UPDATE grant
        SET
            name = ?1,
            funder = ?2,
            call_name = ?3,
            status = ?4,
            amount_requested = ?5,
            amount_received = ?6,
            currency = ?7,
            deadline = ?8,
            submitted_at = ?9,
            decision_at = ?10,
            notes = ?11,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?12
        "#,
        params![
            &grant.name,
            &grant.funder,
            &grant.call_name,
            grant.status.as_str(),
            grant.amount_requested,
            grant.amount_received,
            &grant.currency,
            &grant.deadline,
            &grant.submitted_at,
            &grant.decision_at,
            &grant.notes,
            grant.id
        ],
    )?;

    Ok(())
}

pub fn delete_grant_from_database(grant_id: i64) -> Result<()> {
    let conn = open_database()?;

    conn.execute(
        r#"
        DELETE FROM grant
        WHERE id = ?1
        "#,
        params![grant_id],
    )?;

    Ok(())
}

/**
 * Manuscript functions
 * add_manuscript_to_database: Adds a new manuscript to the database
 * get_manuscripts_from_database: Retrieves all manuscripts from the database
 * get_manuscript_by_id_from_database: Retrieves a manuscript by its ID from the database
 */
pub fn add_manuscript_to_database(new_manuscript: NewManuscript) -> Result<Manuscript> {
    let conn = open_database()?;

    conn.execute(
        r#"
        INSERT INTO manuscript (
            title,
            short_name,
            journal,
            status,
            next_action,
            submitted_at,
            decision_at,
            published_at,
            doi,
            notes
        )
        VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10
        )
        "#,
        params![
            &new_manuscript.title,
            &new_manuscript.short_name,
            &new_manuscript.journal,
            new_manuscript.status.as_str(),
            &new_manuscript.next_action,
            &new_manuscript.submitted_at,
            &new_manuscript.decision_at,
            &new_manuscript.published_at,
            &new_manuscript.doi,
            &new_manuscript.notes,
        ],
    )?;

    let id = conn.last_insert_rowid();

    Ok(Manuscript {
        id: id,
        title: new_manuscript.title,
        short_name: new_manuscript.short_name,
        journal: new_manuscript.journal,
        status: new_manuscript.status,
        next_action: new_manuscript.next_action,
        submitted_at: new_manuscript.submitted_at,
        decision_at: new_manuscript.decision_at,
        published_at: new_manuscript.published_at,
        doi: new_manuscript.doi,
        notes: new_manuscript.notes,
    })
}

pub fn get_manuscripts_from_database() -> Result<Vec<Manuscript>> {
    let conn = open_database()?;

    let mut stmt = conn.prepare(
        r#"
        SELECT
            id,
            title,
            short_name,
            journal,
            status,
            next_action,
            submitted_at,
            decision_at,
            published_at,
            doi,
            notes
        FROM manuscript
        "#,
    )?;

    let manuscripts_iter = stmt.query_map([], |row| {
        Ok(Manuscript {
            id: row.get(0)?,
            title: row.get(1)?,
            short_name: row.get(2)?,
            journal: row.get(3)?,
            status: match row.get::<_, String>(4)?.as_str() {
                "Idea" => ManuscriptStatus::Idea,
                "Drafting" => ManuscriptStatus::Drafting,
                "Submitted" => ManuscriptStatus::Submitted,
                "Revision" => ManuscriptStatus::Revision,
                "Accepted" => ManuscriptStatus::Accepted,
                "Published" => ManuscriptStatus::Published,
                "Rejected" => ManuscriptStatus::Rejected,
                _ => panic!("Invalid manuscript status"),
            },
            next_action: row.get(5)?,
            submitted_at: row.get(6)?,
            decision_at: row.get(7)?,
            published_at: row.get(8)?,
            doi: row.get(9)?,
            notes: row.get(10)?,
        })
    })?;

    let manuscripts: Vec<Manuscript> = manuscripts_iter.collect::<Result<Vec<_>, _>>()?;

    Ok(manuscripts)
}

pub fn get_manuscript_by_id_from_database(manuscript_id: i64) -> Result<Manuscript> {
    let conn = open_database()?;

    let mut stmt = conn.prepare(
        r#"
        SELECT
            id,
            title,
            short_name,
            journal,
            status,
            next_action,
            submitted_at,
            decision_at,
            published_at,
            doi,
            notes
        FROM manuscript
        WHERE id = ?1
        "#,
    )?;

    let manuscript_row = stmt.query_row(params![manuscript_id], |row| {
        Ok(Manuscript {
            id: row.get(0)?,
            title: row.get(1)?,
            short_name: row.get(2)?,
            journal: row.get(3)?,
            status: match row.get::<_, String>(4)?.as_str() {
                "Idea" => ManuscriptStatus::Idea,
                "Drafting" => ManuscriptStatus::Drafting,
                "Submitted" => ManuscriptStatus::Submitted,
                "Revision" => ManuscriptStatus::Revision,
                "Accepted" => ManuscriptStatus::Accepted,
                "Published" => ManuscriptStatus::Published,
                "Rejected" => ManuscriptStatus::Rejected,
                _ => panic!("Invalid manuscript status"),
            },
            next_action: row.get(5)?,
            submitted_at: row.get(6)?,
            decision_at: row.get(7)?,
            published_at: row.get(8)?,
            doi: row.get(9)?,
            notes: row.get(10)?,
        })
    })?;

    Ok(manuscript_row)
}

pub fn update_manuscript_in_database(manuscript: Manuscript) -> Result<()> {
    let conn = open_database()?;

    conn.execute(
        r#"
        UPDATE manuscript
        SET
            title = ?1,
            short_name = ?2,
            journal = ?3,
            status = ?4,
            next_action = ?5,
            submitted_at = ?6,
            decision_at = ?7,
            published_at = ?8,
            doi = ?9,
            notes = ?10,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?11
        "#,
        params![
            &manuscript.title,
            &manuscript.short_name,
            &manuscript.journal,
            manuscript.status.as_str(),
            &manuscript.next_action,
            &manuscript.submitted_at,
            &manuscript.decision_at,
            &manuscript.published_at,
            &manuscript.doi,
            &manuscript.notes,
            manuscript.id
        ],
    )?;

    Ok(())
}

pub fn delete_manuscript_from_database(manuscript_id: i64) -> Result<()> {
    let conn = open_database()?;

    conn.execute(
        r#"
        DELETE FROM manuscript
        WHERE id = ?1
        "#,
        params![manuscript_id],
    )?;

    Ok(())
}
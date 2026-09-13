use diesel::connection::SimpleConnection;
use diesel::{prelude::*};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::sqlite::Sqlite;
use diesel::sql_types::Integer;
use std::path::Path;
use crate::schema::grant;
use crate::schema::manuscript;
use crate::models::{
    Grant, 
    GrantQuery, 
    GrantSort, 
    GrantStatus, 
    NewGrant, 
    SortDirection,
    Manuscript,
    ManuscriptQuery,
    ManuscriptSort,
    ManuscriptStatus,
    NewManuscript
};

/*
    Connection and init
*/

const MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("migrations");

pub fn establish_connection(database_url: &str) -> Result<SqliteConnection, String> {
    let mut connection =
        SqliteConnection::establish(database_url)
            .map_err(|e| e.to_string())?;

    connection
        .batch_execute("PRAGMA foreign_keys = ON;")
        .map_err(|e| e.to_string())?;

    Ok(connection)
}

pub fn initialize_database(database_url: &str) -> Result<(), String> {
    let database_path = Path::new(database_url);

    if let Some(parent) = database_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        {
            std::fs::create_dir_all(parent)
                .map_err(|e| {
                format!("Failed to create database directory '{}': {e}", parent.display())
            })?;
        }

    let mut connection = establish_connection(database_url)
        .map_err(|e| {
            format!(
                "Failed to open database '{}': {e}",
                database_path.display()
            )
        })?;

    let legacy_version =
        get_legacy_database_version(&mut connection)?;

    match legacy_version {
        0 => {
            // Fresh database or already managed by Diesel.
        }

        1 => {
            // Database created by the old Rusqlite initializer.
            verify_legacy_schema(&mut connection)?;
        }

        version => {
            return Err(format!(
                "Unsupported legacy database version: {version}"
            ));
        }
    }

    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| {
            format!("Failed to run migrations: {e}")
        })?;

    if legacy_version == 1 {
        connection
            .batch_execute("PRAGMA user_version = 0;")
            .map_err(|e| {
                format!(
                    "Failed to clear the legacy database version: {e}"
                )
            })?;
    }

    Ok(())
}

/*
    Legacy SQLite database conversion
*/
#[derive(QueryableByName)]
struct UserVersion {
    #[diesel(sql_type = Integer)]
    user_version: i32,
}

fn get_legacy_database_version(connection: &mut SqliteConnection,) -> Result<i32, String> {
    diesel::sql_query("PRAGMA user_version")
        .get_result::<UserVersion>(connection)
        .map(|result| result.user_version)
        .map_err(|e| e.to_string())
}

fn verify_legacy_schema(connection: &mut SqliteConnection,) -> Result<(), String> {
    connection
        .batch_execute(
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
                notes,
                created_at,
                updated_at
            FROM grant
            LIMIT 0;

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
                notes,
                created_at,
                updated_at
            FROM manuscript
            LIMIT 0;

            SELECT
                grant_id,
                manuscript_id
            FROM grant_manuscript
            LIMIT 0;
            "#,
        )
        .map_err(|e| {
            format!(
                "The existing database claims to be N4G schema version 1, \
                 but its structure does not match that version: {e}"
            )
        })
}

/*
    Database functions for Grants
*/

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = grant)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GrantRow {
    pub id: Option<i64>,
    pub name: String,
    pub funder: String,
    pub call_name: Option<String>,
    pub status: String,
    pub amount_requested: Option<i64>,
    pub amount_received: Option<i64>,
    pub currency: String,
    pub deadline: Option<String>,
    pub submitted_at: Option<String>,
    pub decision_at: Option<String>,
    pub notes: Option<String>,
}

impl TryFrom<GrantRow> for Grant {
    type Error = String;

    fn try_from(row: GrantRow) -> Result<Self, Self::Error> {
        Ok(Grant {
            id: row.id.ok_or("Missing id")? as i64,
            name: row.name,
            funder: row.funder,
            call_name: row.call_name,
            status: GrantStatus::try_from(row.status)?,
            amount_requested: row.amount_requested.map(|v| v as i64),
            amount_received: row.amount_received.map(|v| v as i64),
            currency: row.currency,
            deadline: row.deadline,
            submitted_at: row.submitted_at,
            decision_at: row.decision_at,
            notes: row.notes,
        })
    }
}

pub fn get_all_grants(database_url: &str) -> Result<Vec<Grant>, String> {
    let mut connection = establish_connection(database_url).map_err(|e| e.to_string())?;

    let rows = grant::table
        .select(GrantRow::as_select())
        .load::<GrantRow>(&mut connection)
        .map_err(|e| e.to_string())?;

    rows.into_iter()
        .map(Grant::try_from)
        .collect()
}

pub fn update_grant(database_url: &str, grant: &Grant) -> Result<(), String> {
    let mut connection = establish_connection(database_url).map_err(|e| e.to_string())?;

    diesel::update(grant::table.filter(grant::id.eq(Some(grant.id as i64))))
        .set((
            grant::name.eq(&grant.name),
            grant::funder.eq(&grant.funder),
            grant::call_name.eq(&grant.call_name),
            grant::status.eq(grant.status.as_str()),
            grant::amount_requested.eq(grant.amount_requested.map(|v| v as i64)),
            grant::amount_received.eq(grant.amount_received.map(|v| v as i64)),
            grant::currency.eq(&grant.currency),
            grant::deadline.eq(&grant.deadline),
            grant::submitted_at.eq(&grant.submitted_at),
            grant::decision_at.eq(&grant.decision_at),
            grant::notes.eq(&grant.notes),
        ))
        .execute(&mut connection)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn delete_grant(database_url: &str, grant_id: i64) -> Result<(), String> {
    let mut connection = establish_connection(database_url).map_err(|e| e.to_string())?;

    diesel::delete(grant::table.filter(grant::id.eq(Some(grant_id as i64))))
        .execute(&mut connection)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn add_grant(database_url: &str, new_grant: &NewGrant) -> Result<Grant, String> {
    let mut connection = establish_connection(database_url).map_err(|e| e.to_string())?;

    diesel::insert_into(grant::table)
        .values((
            grant::name.eq(&new_grant.name),
            grant::funder.eq(&new_grant.funder),
            grant::call_name.eq(&new_grant.call_name),
            grant::status.eq(new_grant.status.as_str()),
            grant::amount_requested.eq(new_grant.amount_requested),
            grant::amount_received.eq(new_grant.amount_received),
            grant::currency.eq(&new_grant.currency),
            grant::deadline.eq(&new_grant.deadline),
            grant::submitted_at.eq(&new_grant.submitted_at),
            grant::decision_at.eq(&new_grant.decision_at),
            grant::notes.eq(&new_grant.notes),
        ))
        .execute(&mut connection)
        .map_err(|e| e.to_string())?;

    let inserted_id: i64 = diesel::select(
        diesel::dsl::sql::<diesel::sql_types::BigInt>("last_insert_rowid()"),
    )
    .get_result(&mut connection)
    .map_err(|e| e.to_string())?;
    let inserted_grant: GrantRow = grant::table
        .filter(grant::id.eq(Some(inserted_id)))
        .select(GrantRow::as_select())
        .first(&mut connection)
        .map_err(|e| e.to_string())?;

    Grant::try_from(inserted_grant)
}

pub fn get_filtered_grants(database_url: &str, request: &GrantQuery) -> Result<Vec<Grant>, String> {
    let mut connection = establish_connection(database_url).map_err(|e| e.to_string())?;

    let mut statement = grant::table.into_boxed::<Sqlite>();

    // ID filtering
    if let Some(id) = request.id {
        statement = statement.filter(grant::id.eq(Some(id as i64)));
    }

    // Status filtering
    if !request.statuses.is_empty() {
        let statuses: Vec<&str> = request
            .statuses
            .iter()
            .map(GrantStatus::as_str)
            .collect();

        statement = statement.filter(
            grant::status.eq_any(statuses)
        );
    }

    // Funder filtering
    if let Some(funder) = request
        .funder
        .as_deref()
        .map(str::trim)
        .filter(|funder| !funder.is_empty())
    {
        let like_pattern = format!("%{}%", funder);

        statement = statement.filter(
            grant::funder.like(like_pattern)
        );
    }

    // Search term filtering
    if let Some(search_term) = request.search.as_deref().map(str::trim).filter(|search|!search.is_empty()) {
        let like_pattern = format!("%{}%", search_term);
        
        statement = statement.filter(
            grant::name.like(like_pattern.clone())
                .or(grant::funder.like(like_pattern.clone()))
                .or(grant::call_name.like(like_pattern)));
    }

    // Sorting
    statement = match (&request.sort_by, &request.sort_direction,) {
        (GrantSort::Deadline,   SortDirection::Asc) =>  {statement.order(grant::deadline.asc())}
        (GrantSort::Deadline,   SortDirection::Desc) => {statement.order(grant::deadline.desc())}
        (GrantSort::Name,       SortDirection::Asc) =>  {statement.order(grant::name.asc())}
        (GrantSort::Name,       SortDirection::Desc) => {statement.order(grant::name.desc())}
        (GrantSort::Status,     SortDirection::Asc) =>  {statement.order(grant::status.asc())}
        (GrantSort::Status,     SortDirection::Desc) => {statement.order(grant::status.desc())}
        (GrantSort::Funder,     SortDirection::Asc) =>  {statement.order(grant::funder.asc())}
        (GrantSort::Funder,     SortDirection::Desc) => {statement.order(grant::funder.desc())}
    };

    let rows = statement
        .select(GrantRow::as_select())
        .load::<GrantRow>(&mut connection)
        .map_err(|e| e.to_string())?;

    rows.into_iter()
        .map(Grant::try_from)
        .collect()
}

pub fn get_grant_by_id(database_url: &str, grant_id: i64) -> Result<Option<Grant>, String> {
    let mut connection = establish_connection(database_url).map_err(|e| e.to_string())?;

    let row = grant::table
        .filter(grant::id.eq(Some(grant_id as i64)))
        .select(GrantRow::as_select())
        .first::<GrantRow>(&mut connection)
        .optional()
        .map_err(|e| e.to_string())?;

    match row {
        Some(grant_row) => Ok(Some(Grant::try_from(grant_row)?)),
        None => Ok(None),
    }
}

/*
    Database functions for Manuscripts
*/

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = manuscript)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ManuscriptRow {
    pub id: Option<i64>,
    pub title: String,
    pub short_name: Option<String>,
    pub journal: Option<String>,
    pub status: String,
    pub next_action: Option<String>,
    pub submitted_at: Option<String>,
    pub decision_at: Option<String>,
    pub published_at: Option<String>,
    pub doi: Option<String>,
    pub notes: Option<String>,
}

impl TryFrom<ManuscriptRow> for Manuscript {
    type Error = String;

    fn try_from(row: ManuscriptRow) -> Result<Self, Self::Error> {
        Ok(Manuscript {
            id: row.id.ok_or("Missing id")? as i64,
            title: row.title,
            short_name: row.short_name,
            journal: row.journal,
            status: ManuscriptStatus::try_from(row.status)?,
            next_action: row.next_action,
            submitted_at: row.submitted_at,
            decision_at: row.decision_at,
            published_at: row.published_at,
            doi: row.doi,
            notes: row.notes,
        })
    }
}

pub fn get_all_manuscripts(database_url: &str) -> Result<Vec<Manuscript>, String> {
    let mut connection = establish_connection(database_url).map_err(|e| e.to_string())?;

    let rows = manuscript::table
        .order(manuscript::title.asc())
        .select(ManuscriptRow::as_select())
        .load::<ManuscriptRow>(&mut connection)
        .map_err(|e| e.to_string())?;

    rows.into_iter()
        .map(Manuscript::try_from)
        .collect()
}

pub fn update_manuscript(database_url: &str, manuscript: &Manuscript) -> Result<(), String> {
    let mut connection = establish_connection(database_url).map_err(|e| e.to_string())?;

    diesel::update(manuscript::table.filter(manuscript::id.eq(Some(manuscript.id as i64))))
        .set((
            manuscript::title.eq(&manuscript.title),
            manuscript::short_name.eq(&manuscript.short_name),
            manuscript::journal.eq(&manuscript.journal),
            manuscript::status.eq(manuscript.status.as_str()),
            manuscript::next_action.eq(&manuscript.next_action),
            manuscript::submitted_at.eq(&manuscript.submitted_at),
            manuscript::decision_at.eq(&manuscript.decision_at),
            manuscript::published_at.eq(&manuscript.published_at),
            manuscript::doi.eq(&manuscript.doi),
            manuscript::notes.eq(&manuscript.notes),
        ))
        .execute(&mut connection)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn delete_manuscript(database_url: &str, manuscript_id: i64) -> Result<(), String> {
    let mut connection = establish_connection(database_url).map_err(|e| e.to_string())?;

    diesel::delete(manuscript::table.filter(manuscript::id.eq(Some(manuscript_id))))
        .execute(&mut connection)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn add_manuscript(database_url: &str, new_manuscript: &NewManuscript) -> Result<Manuscript, String> {
    let mut connection = establish_connection(database_url).map_err(|e| e.to_string())?;

    diesel::insert_into(manuscript::table)
        .values((
            manuscript::title.eq(&new_manuscript.title),
            manuscript::short_name.eq(&new_manuscript.short_name),
            manuscript::journal.eq(&new_manuscript.journal),
            manuscript::status.eq(new_manuscript.status.as_str()),
            manuscript::next_action.eq(&new_manuscript.next_action),
            manuscript::submitted_at.eq(&new_manuscript.submitted_at),
            manuscript::decision_at.eq(&new_manuscript.decision_at),
            manuscript::published_at.eq(&new_manuscript.published_at),
            manuscript::doi.eq(&new_manuscript.doi),
            manuscript::notes.eq(&new_manuscript.notes),
        ))
        .execute(&mut connection)
        .map_err(|e| e.to_string())?;

    let inserted_id: i64 = diesel::select(
        diesel::dsl::sql::<diesel::sql_types::BigInt>("last_insert_rowid()"),
    )
    .get_result(&mut connection)
    .map_err(|e| e.to_string())?;
    let inserted_manuscript: ManuscriptRow = manuscript::table
        .filter(manuscript::id.eq(Some(inserted_id)))
        .select(ManuscriptRow::as_select())
        .first(&mut connection)
        .map_err(|e| e.to_string())?;

    Manuscript::try_from(inserted_manuscript)
}

pub fn get_filtered_manuscripts(database_url: &str, request: &ManuscriptQuery) -> Result<Vec<Manuscript>, String> {
    let mut connection = establish_connection(database_url).map_err(|e| e.to_string())?;

    let mut statement = manuscript::table.into_boxed::<Sqlite>();

    // ID filtering
    if let Some(id) = request.id {
        statement = statement.filter(manuscript::id.eq(Some(id as i64)));
    }

    // Status filtering
    if !request.statuses.is_empty() {
        let statuses: Vec<&str> = request
            .statuses
            .iter()
            .map(|s| s.as_str())
            .collect();
        statement = statement.filter(manuscript::status.eq_any(statuses));
    }

    // Search term filtering
    if let Some(search_term) = request.search.as_deref().map(str::trim)
        .filter(|search| !search.is_empty()) {
        let like_pattern = format!("%{}%", search_term);
        statement = statement.filter(
            manuscript::title.like(like_pattern.clone())
                .or(manuscript::short_name.like(like_pattern.clone()))
                .or(manuscript::journal.like(like_pattern.clone()))
                .or(manuscript::doi.like(like_pattern.clone()))
        );
    }

    // Journal filtering
    if let Some(journal) = request
        .journal
        .as_deref()
        .map(str::trim)
        .filter(|journal| !journal.is_empty())
    {
        let like_pattern = format!("%{}%", journal);

        statement = statement.filter(
            manuscript::journal.like(like_pattern)
        );
    }

    // Sorting
    statement = match (&request.sort_by, &request.sort_direction) {
        (ManuscriptSort::Title,     SortDirection::Asc) =>  statement.order(manuscript::title.asc()),
        (ManuscriptSort::Title,     SortDirection::Desc) => statement.order(manuscript::title.desc()),
        (ManuscriptSort::Status,    SortDirection::Asc) =>  statement.order(manuscript::status.asc()),
        (ManuscriptSort::Status,    SortDirection::Desc) => statement.order(manuscript::status.desc()),
        (ManuscriptSort::Journal,   SortDirection::Asc) =>  statement.order(manuscript::journal.asc()),
        (ManuscriptSort::Journal,   SortDirection::Desc) => statement.order(manuscript::journal.desc()),
    };

    let manuscripts: Vec<ManuscriptRow> = statement
        .select(ManuscriptRow::as_select())
        .load(&mut connection)
        .map_err(|e| e.to_string())?;

    manuscripts.into_iter().map(Manuscript::try_from).collect()
}

pub fn get_manuscript_by_id(database_url: &str, manuscript_id: i64) -> Result<Option<Manuscript>, String> {
    let mut connection = establish_connection(database_url).map_err(|e| e.to_string())?;

    let row = manuscript::table
        .filter(manuscript::id.eq(Some(manuscript_id as i64)))
        .select(ManuscriptRow::as_select())
        .first::<ManuscriptRow>(&mut connection)
        .optional()
        .map_err(|e| e.to_string())?;

    match row {
        Some(manuscript_row) => Ok(Some(Manuscript::try_from(manuscript_row)?)),
        None => Ok(None),
    }
}
use diesel::{connection, prelude::*};
use diesel::sqlite::Sqlite;
use num_bigint::BigUint;
use crate::schema::grant;
use crate::models::{Grant, GrantQuery, GrantSort, GrantStatus, NewGrant, SortDirection};

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

pub fn establish_connection(database_url: &str) -> Result<SqliteConnection, diesel::ConnectionError> {
    SqliteConnection::establish(database_url)
}

pub fn get_all_grants(database_url: &str) -> Result<Vec<Grant>, String> {
    let mut connection = establish_connection(database_url).map_err(|e| e.to_string())?;

    let rows = grant::table
        .order(grant::name.asc())
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
    if !request.funders.is_empty() {
    let funders: Vec<&str> = request
        .funders
        .iter()
        .map(String::as_str)
        .collect();

    statement = statement.filter(
        grant::funder.eq_any(funders)
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
    statement = match (&request.sort_by, &request.direction,) {
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
        .order(grant::name.asc())
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
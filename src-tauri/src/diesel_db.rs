use diesel::{connection, prelude::*};
use diesel::sqlite::Sqlite;
use crate::schema::grant;
use crate::models::{Grant, GrantQuery, GrantSort, GrantStatus, SortDirection};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = grant)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GrantRow {
    pub id: Option<i32>,
    pub name: String,
    pub funder: String,
    pub call_name: Option<String>,
    pub status: String,
    pub amount_requested: Option<i32>,
    pub amount_received: Option<i32>,
    pub currency: String,
    pub deadline: Option<String>,
    pub submitted_at: Option<String>,
    pub decision_at: Option<String>,
    pub notes: Option<String>,
}

impl TryFrom<GrantRow> for Grant {
    type Error = String;

    fn try_from(row: GrantRow) -> Result<Self, Self::Error> {
        type Error = String;

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

pub fn get_filtered_grants(database_url: &str, request: &GrantQuery) -> Result<Vec<Grant>, String> {
    let mut connection = establish_connection(database_url).map_err(|e| e.to_string())?;

    let mut statement = grant::table.into_boxed::<Sqlite>();

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
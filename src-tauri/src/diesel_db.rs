use diesel::prelude::*;
use crate::schema::grant;
use crate::models::{Grant, GrantStatus};

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

pub fn get_accepted_grants(database_url: &str) -> Result<Vec<Grant>, String> {
    let mut connection = establish_connection(database_url)
        .map_err(|error| error.to_string())?;

    let rows = grant::table
        .filter(grant::status.eq("Accepted"))
        .order(grant::name.asc())
        .select(GrantRow::as_select())
        .load::<GrantRow>(&mut connection)
        .map_err(|e| e.to_string())?;

    rows.into_iter()
        .map(Grant::try_from)
        .collect()
}
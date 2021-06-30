use chrono::{NaiveDateTime};
use crate::schema::authentication_entries;
use serde::{Serialize, Deserialize};

#[derive(Queryable, QueryableByName, Insertable, Serialize, Deserialize, Debug)]
#[table_name="authentication_entries"]
pub struct AuthenticationEntry {
    pub id: uuid::Uuid,
    pub account_name: String,
    pub secret: uuid::Uuid,
    pub policy: serde_json::Value, 
    pub policy_base64: String,
    pub valid_until: NaiveDateTime,
    pub blockchain_index: Option<i64>
}

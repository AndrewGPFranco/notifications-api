use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid as uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DemandDTO {
    pub id_usuario: i32,
    pub id_demanda: uuid,
    pub titulo_tarefa: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct OutputNotificationDTO {
    pub(crate) content: String,
    pub(crate) was_it_viewed: bool,
    pub(crate) created_at: NaiveDate,
}
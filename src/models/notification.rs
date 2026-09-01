use chrono::NaiveDate;
use uuid::Uuid as uuid;

#[allow(dead_code)]
pub struct Notification {
    pub(crate) id: i32,
    pub(crate) user_id: i32,
    pub(crate) content: String,
    pub(crate) demand_id: uuid,
    pub(crate) category: String,
    pub(crate) was_it_viewed: bool,
    pub(crate) created_at: NaiveDate,
    pub(crate) updated_at: Option<NaiveDate>,
}

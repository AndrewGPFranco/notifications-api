use chrono::NaiveDate;

#[allow(dead_code)]
pub struct Notification {
    id: i32,
    user_id: i32,
    content: String,
    category: Category,
    was_it_viewed: bool,
    created_at: NaiveDate,
    updated_at: Option<NaiveDate>
}

#[allow(dead_code)]
enum Category {
    TaskExpired,
    TaskCloseToExpire,
}
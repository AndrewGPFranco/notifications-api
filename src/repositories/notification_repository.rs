use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid as uuid;

use crate::dtos::notification_dto::DemandDTO;
use crate::models::notification::Notification;

#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn create(&self, demand: DemandDTO, content: String) -> Result<Notification, sqlx::Error>;
    async fn get_by_id_demand(&self, id_demand: uuid) -> Result<Option<Notification>, sqlx::Error>;
    async fn get_by_user(&self, id_user: i32) -> Result<Vec<Notification>, sqlx::Error>;
}

pub struct PostgresNotificationRepository {
    pool: sqlx::PgPool,
}

impl PostgresNotificationRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NotificationRepository for PostgresNotificationRepository {
    async fn create(&self, demand: DemandDTO, content: String) -> Result<Notification, sqlx::Error> {
        let notification = sqlx::query_as!(
            Notification,
            r#"
            INSERT INTO notifications (user_id, demand_id, content, category, was_it_viewed, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, demand_id, content, category, was_it_viewed, created_at, updated_at
            "#,
            demand.id_usuario,
            demand.id_demanda,
            content,
            "TASK_CLOSE_TO_EXPIRE",
            false,
            Utc::now().date_naive(),
            None::<chrono::NaiveDate>
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(notification)
    }

    async fn get_by_id_demand(&self, id_demand: uuid) -> Result<Option<Notification>, sqlx::Error> {
        let notification = sqlx::query_as!(
            Notification,
            r#"
            SELECT * FROM notifications WHERE demand_id = $1
            "#,
            id_demand
        ).fetch_optional(&self.pool).await?;
        
        Ok(notification)
    }

    async fn get_by_user(&self, id_user: i32) -> Result<Vec<Notification>, sqlx::Error> {
        let notifications = sqlx::query_as!(
            Notification,
            r#"
            SELECT * FROM notifications WHERE user_id = $1
            "#,
            id_user
        ).fetch_all(&self.pool).await?;
        
        Ok(notifications)
    }
}

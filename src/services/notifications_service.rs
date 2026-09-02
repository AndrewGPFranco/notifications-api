use chrono::{DateTime, Days, FixedOffset, Utc};
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::dtos::notification_dto::{DemandDTO, OutputNotificationDTO};
use crate::state::AppState;

const TEMPO: &str = "0 0 * * * *";

#[allow(dead_code)]
const TEMPO_TESTE: &str = "*/5 * * * * *";

pub async fn setup_cron(state: AppState) -> anyhow::Result<JobScheduler> {
    let scheduler = JobScheduler::new().await?;

    scheduler
        .add(Job::new_async(TEMPO, move |_uuid, _lock| {
            let state = state.clone();

            Box::pin(async move {
                process_notifications(&state).await;
            })
        })?)
        .await?;

    scheduler.start().await?;
    Ok(scheduler)
}

pub async fn process_notifications(state: &AppState) {
    let now = get_now();
    println!("Iniciando processo de notificações {}", now);

    match recover_demands(&state.admin_api_key).await {
        Ok(demands) => {
            handler_notifications(demands, state).await;
        }
        Err(error) => {
            eprintln!("Erro ao recuperar as demandas: {error}");
        }
    }
}

fn get_now() -> DateTime<FixedOffset> {
    let fuso_brasilia = FixedOffset::west_opt(3 * 3600).unwrap();
    let agora_brasilia = Utc::now().with_timezone(&fuso_brasilia);
    agora_brasilia
}

async fn handler_notifications(demands: Vec<DemandDTO>, state: &AppState) {
    for demand in demands {
        let created_at = demand.created_at;

        let fifteen_days_ago = get_now() - Days::new(15);

        if created_at < fifteen_days_ago.naive_local() {
            if let Err(error) = create_notification(demand, state).await {
                eprintln!("Erro ao criar notificação: {error}");
            }
        }
    }
}

async fn recover_demands(api_key: &str) -> Result<Vec<DemandDTO>, anyhow::Error> {
    let notifications = reqwest::Client::new()
        .get("http://localhost:9001/api/admin/all-demands")
        .header("X-API-Key", api_key)
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<DemandDTO>>()
        .await?;

    Ok(notifications)
}

async fn create_notification(demand: DemandDTO, state: &AppState) -> Result<(), sqlx::Error> {
    let notification = state.notification_repo.get_by_id_demand(demand.id_demanda).await?;

    if notification.is_none() {
        // TODO: tratar outros cenários possíveis
        let content = format!("Tarefa expirada: {}", demand.titulo_tarefa);
        state.notification_repo.create(demand, content).await?;
    }

    Ok(())
}

pub async fn recover_notifications_user(id_user: i32, state: &AppState) -> Result<Vec<OutputNotificationDTO>, sqlx::Error> {
    let notifications = state.notification_repo.get_by_user(id_user).await?;

    let output = notifications
        .iter()
        .map(|notification| OutputNotificationDTO {
            content: notification.content.clone(),
            was_it_viewed: notification.was_it_viewed,
            created_at: notification.created_at,
        })
        .collect();

    Ok(output)
}
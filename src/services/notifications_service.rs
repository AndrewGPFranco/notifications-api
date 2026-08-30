use tokio_cron_scheduler::{JobScheduler, Job};

const TEMPO: &str = "0 0 * * * *";

pub async fn setup_cron() -> anyhow::Result<JobScheduler> {
    let scheduler = JobScheduler::new().await?;

    scheduler.add(
        Job::new_async(TEMPO, |_uuid, _1| {
            Box::pin(async move {
                println!("rodando job de hora em hora");
                recuperar_notificacoes().await;
            })
        })?).await?;

    scheduler.start().await?;
    Ok(scheduler)
}

async fn recuperar_notificacoes() {
    println!("Buscando notificações");
}
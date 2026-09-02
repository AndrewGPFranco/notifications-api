use std::env;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Conexão ao banco de dados estabelecida com sucesso!");

    sqlx::migrate!().run(&pool).await?;

    println!("Migrations executadas com sucesso!");

    Ok(pool)
}

use std::env;
use dotenv::dotenv;
use sqlx::PgPool;

pub async fn connect() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPool::connect(&database_url).await.unwrap();
    
    println!("Conexão ao banco de dados estabelecida com sucesso!");
}
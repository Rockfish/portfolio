use crate::config::get_config;
use crate::read_history_orm::get_bought_records;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use sqlx::PgPool;
use std::time::Duration;

mod config;
mod read_history_orm;

mod entities;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let config = get_config();
    let pool = PgPool::connect(&config.db_connection_string).await.unwrap();

    let db = get_db_connection(&config.db_connection_string).await.unwrap();

    println!("db connection is ok: {}", db.ping().await.is_ok());

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    println!("migration completed");

    get_bought_records(&db, &pool, "RIO").await;

    Ok(())
}

async fn get_db_connection(connection_string: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(connection_string); //"protocol://username:password@host/database");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public"); // Setting default PostgreSQL schema

    Database::connect(opt).await
}

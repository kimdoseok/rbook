use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use std::env;

pub struct AppState {
    pub pool: Pool<Postgres>,
}

pub struct Navigation {
    pub page_length: i32,
    pub page_number: i32,
    pub page_current: i32,
}

impl AppState {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

pub async fn get_state() -> Result<AppState, sqlx::Error> {
    dotenvy::dotenv().ok();
    let _pghost: String = env::var("DB_PGSQL_HOST").unwrap_or_else(|_| "localhost".into());
    let _pguser: String = env::var("DB_PGSQL_USER").unwrap_or_else(|_| "doseok".into());
    let _pgpassword: String = env::var("DB_PGSQL_PASSWORD").unwrap_or_else(|_| "kim7795004".into());
    let _pgdbname: String = env::var("DB_PGSQL_DBNAME").unwrap_or_else(|_| "rbook".into());
    let _pgport: String = env::var("DB_PGSQL_PORT").unwrap_or_else(|_| "5432".into());
    let pgurl: String = format!("postgresql://{}:{}@{}:{}/{}", _pguser, _pgpassword, _pghost, _pgport, _pgdbname);
    let connmax: u32 = env::var("DB_PGSQL_CONN_MAX").unwrap_or_else(|_| "20".into()).parse().unwrap_or(20);
    let connmin: u32 = env::var("DB_PGSQL_CONN_MIN").unwrap_or_else(|_| "5".into()).parse().unwrap_or(5);
    let connacquire: u64 = env::var("DB_PGSQL_CONN_ACQUIRE_TIMEOUT").unwrap_or_else(|_| "3".into()).parse().unwrap_or(3);
    let connidle: u64 = env::var("DB_PGSQL_CONN_IDLE_TIMEOUT").unwrap_or_else(|_| "600".into()).parse().unwrap_or(600);

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(connmax)
        .min_connections(connmin)
        .acquire_timeout(Duration::from_secs(connacquire))
        .idle_timeout(Duration::from_secs(connidle))
        .connect(&pgurl)
        .await
        .map_err(|err| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("failed to connect to database: {err}"))))?;

    Ok(AppState::new(pool))
}

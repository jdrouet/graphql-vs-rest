use sqlx::postgres::PgPoolOptions;

pub type Pool = sqlx::postgres::PgPool;
pub type Connection = sqlx::postgres::PgConnection;

pub async fn create_pool() -> Pool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres@localhost/postgres".into());
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap()
}

use sqlx::postgres::PgPoolOptions;

pub type Pool = sqlx::postgres::PgPool;
pub type Connection = sqlx::postgres::PgConnection;

pub async fn create_pool() -> Pool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres@database/postgres")
        .await
        .unwrap()
}

use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[cfg_attr(
    feature = "graphql",
    derive(async_graphql::InputObject, async_graphql::SimpleObject)
)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: Uuid,
    pub content: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageWithCreator {
    pub id: Uuid,
    pub content: String,
    pub creator: super::account::Account,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::InputObject))]
pub struct CreateMessage {
    pub content: String,
}

impl CreateMessage {
    pub async fn persist<'e, E: sqlx::PgExecutor<'e>>(
        &self,
        executor: E,
        creator_id: Uuid,
    ) -> Result<Message, sqlx::Error> {
        sqlx::query_as(
            r#"
INSERT INTO messages(content, created_by)
VALUES ($1,$2)
RETURNING id, content, created_by, created_at
"#,
        )
        .bind(self.content.as_str())
        .bind(&creator_id)
        .fetch_one(executor)
        .await
    }
}

impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for Message {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Message {
            id: row.try_get(0)?,
            content: row.try_get(1)?,
            created_by: row.try_get(2)?,
            created_at: row.try_get(3)?,
        })
    }
}

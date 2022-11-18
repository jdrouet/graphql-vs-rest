use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "rest", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: Uuid,
    pub content: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
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

#[cfg(feature = "graphql")]
#[async_graphql::Object]
impl Message {
    async fn id(&self) -> &Uuid {
        &self.id
    }

    async fn content(&self) -> &str {
        &self.content
    }

    async fn created_by(&self) -> &Uuid {
        &self.created_by
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn creator<'ctx>(
        &self,
        ctx: &async_graphql::Context<'ctx>,
    ) -> async_graphql::Result<crate::model::account::Account> {
        let pool = ctx.data_unchecked::<crate::service::database::Pool>();
        let mut conn = pool.acquire().await?;
        let res = crate::model::account::Account::find_by_id(&mut conn, &self.created_by).await?;
        Ok(res)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "rest", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct MessageWithCreator {
    pub id: Uuid,
    pub content: String,
    pub creator: super::account::Account,
    pub created_at: DateTime<Utc>,
}

impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for MessageWithCreator {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(MessageWithCreator {
            id: row.try_get(0)?,
            content: row.try_get(1)?,
            created_at: row.try_get(2)?,
            creator: super::account::Account {
                id: row.try_get(3)?,
                name: row.try_get(4)?,
                email: row.try_get(5)?,
                created_at: row.try_get(6)?,
            },
        })
    }
}

#[derive(Clone, Debug, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "rest", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CreateMessage {
    pub creator_id: Uuid,
    #[validate(length(min = 1))]
    pub content: String,
}

impl CreateMessage {
    pub async fn persist<'e, E: sqlx::PgExecutor<'e>>(
        &self,
        executor: E,
    ) -> Result<Message, sqlx::Error> {
        sqlx::query_as(
            r#"
INSERT INTO messages(content, created_by)
VALUES ($1,$2)
RETURNING id, content, created_by, created_at
"#,
        )
        .bind(self.content.as_str())
        .bind(&self.creator_id)
        .fetch_one(executor)
        .await
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "rest", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ListMessages {
    count: Option<u64>,
    page: Option<u64>,
}

impl ListMessages {
    pub async fn fetch<'e, E: sqlx::PgExecutor<'e>>(
        &self,
        executor: E,
    ) -> Result<Vec<Message>, sqlx::Error> {
        let count = self.count.unwrap_or(50) as i64;
        let page = self.page.unwrap_or(0) as i64;
        let offset = count * page;

        sqlx::query_as(
            r#"
SELECT id, content, created_by, created_at
FROM messages
ORDER BY created_at
LIMIT $1
OFFSET $2
"#,
        )
        .bind(count)
        .bind(offset)
        .fetch_all(executor)
        .await
    }

    pub async fn fetch_with_creator<'e, E: sqlx::PgExecutor<'e>>(
        &self,
        executor: E,
    ) -> Result<Vec<MessageWithCreator>, sqlx::Error> {
        let count = self.count.unwrap_or(50) as i64;
        let page = self.page.unwrap_or(0) as i64;
        let offset = count * page;

        sqlx::query_as(
            r#"
SELECT m.id, m.content, m.created_at, a.id, a.name, a.email, a.created_at
FROM messages m
JOIN accounts a ON m.created_by = a.id
ORDER BY m.created_at
LIMIT $1
OFFSET $2
"#,
        )
        .bind(count)
        .bind(offset)
        .fetch_all(executor)
        .await
    }
}

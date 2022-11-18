use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[cfg_attr(
    feature = "graphql",
    derive(async_graphql::InputObject, async_graphql::SimpleObject)
)]
#[cfg_attr(feature = "rest", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for Account {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Account {
            id: row.try_get(0)?,
            name: row.try_get(1)?,
            email: row.try_get(2)?,
            created_at: row.try_get(3)?,
        })
    }
}
impl Account {
    pub async fn find_by_id<'e, E: sqlx::PgExecutor<'e>>(
        executor: E,
        account_id: &Uuid,
    ) -> Result<Account, sqlx::Error> {
        sqlx::query_as("SELECT id, name, email, created_at FROM accounts WHERE id = $1")
            .bind(account_id)
            .fetch_one(executor)
            .await
    }
}

#[derive(Clone, Debug, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "graphql", derive(async_graphql::InputObject))]
#[cfg_attr(feature = "rest", derive(schemars::JsonSchema))]
pub struct CreateAccount {
    #[validate(length(min = 2))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

impl CreateAccount {
    pub async fn persist<'e, E: sqlx::PgExecutor<'e>>(
        &self,
        executor: E,
    ) -> Result<Account, sqlx::Error> {
        sqlx::query_as(
            r#"
INSERT INTO accounts(name,email)
VALUES ($1,$2)
RETURNING id, name, email, created_at
"#,
        )
        .bind(self.name.as_str())
        .bind(self.email.as_str())
        .fetch_one(executor)
        .await
    }
}

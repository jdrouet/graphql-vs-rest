use async_graphql::{Context, Object};
use common::model::account::{Account, CreateAccount};
use common::service::database::Pool;
use validator::Validate;

#[derive(Default)]
pub struct AccountQuery;

#[Object]
impl AccountQuery {
    async fn accounts(&self) -> bool {
        true
    }
}

#[derive(Default)]
pub struct AccountMutation;

#[Object]
impl AccountMutation {
    async fn create_account<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: CreateAccount,
    ) -> async_graphql::Result<Account> {
        input.validate()?;
        let pool = ctx.data_unchecked::<Pool>();
        let mut conn = pool.acquire().await?;
        let result = input.persist(&mut conn).await?;
        Ok(result)
    }
}

use async_graphql::{Context, Object};
use common::model::message::{CreateMessage, Message};
use common::service::database::Pool;
use validator::Validate;

#[derive(Default)]
pub struct MessageQuery;

#[Object]
impl MessageQuery {
    async fn accounts(&self) -> bool {
        true
    }
}

#[derive(Default)]
pub struct MessageMutation;

#[Object]
impl MessageMutation {
    async fn create_message<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: CreateMessage,
    ) -> async_graphql::Result<Message> {
        input.validate()?;
        let pool = ctx.data_unchecked::<Pool>();
        let mut conn = pool.acquire().await?;
        let result = input.persist(&mut conn).await?;
        Ok(result)
    }
}

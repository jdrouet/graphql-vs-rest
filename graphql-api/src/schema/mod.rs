pub mod account;
pub mod health;
pub mod message;

use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct Query(
    health::HealthQuery,
    account::AccountQuery,
    message::MessageQuery,
);

#[derive(MergedObject, Default)]
pub struct Mutation(account::AccountMutation, message::MessageMutation);

/// Build the GraphQL schema.
pub fn build_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(
        Query::default(),
        Mutation::default(),
        EmptySubscription::default(),
    )
}

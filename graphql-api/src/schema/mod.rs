pub mod health;

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, SchemaBuilder};

pub type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct Query(health::HealthQuery);

/// Build the GraphQL schema.
pub fn build_schema() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    Schema::build(
        Query::default(),
        EmptyMutation::default(),
        EmptySubscription::default(),
    )
}

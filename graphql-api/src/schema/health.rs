use async_graphql::Object;

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    /// Returns `true` to signify that the GraphQL server is reachable.
    async fn running(&self) -> bool {
        true
    }
}

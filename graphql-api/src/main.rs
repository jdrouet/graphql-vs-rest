pub mod schema;

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::Extension, routing::post, Router, Server};

async fn graphql_handler(
    schema: Extension<schema::GraphQLSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() {
    let schema = schema::build_schema().finish();

    let app = Router::new()
        .route("/", post(graphql_handler))
        .layer(Extension(schema));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

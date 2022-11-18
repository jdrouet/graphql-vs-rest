use crate::error::ServerError;
use axum::extract::{Extension, Json, Query};
use common::model::message::{ListMessages, MessageWithCreator};
use common::service::database;

pub async fn handler(
    Extension(database_pool): Extension<database::Pool>,
    Query(payload): Query<ListMessages>,
) -> Result<Json<Vec<MessageWithCreator>>, ServerError> {
    let mut conn = database_pool.acquire().await?;
    let result = payload.fetch_with_creator(&mut conn).await?;
    Ok(Json(result))
}

use crate::error::ServerError;
use axum::{extract::Extension, response::Json};
use common::model::message::{CreateMessage, Message};
use common::service::database;
use validator::Validate;

pub async fn handler(
    Extension(database_pool): Extension<database::Pool>,
    Json(payload): Json<CreateMessage>,
) -> Result<Json<Message>, ServerError> {
    payload.validate()?;
    let mut conn = database_pool.acquire().await?;
    let result = payload.persist(&mut conn).await?;
    Ok(Json(result))
}

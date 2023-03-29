use crate::{
    db::DB,
    response::GenericResponse,
    schema::{CreatePreregSchema},
    WebResult,
};
use warp::{http::StatusCode, reject, reply::json, reply::with_status, Reply};

pub async fn health_checker_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Build CRUD API with Rust and MongoDBssss";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(json(response_json))
}

pub async fn create_preregistration_handler(body: CreatePreregSchema, db: DB) -> WebResult<impl Reply> {
    let result = db.create_prereg(&body).await.map_err(|e| reject::custom(e))?;

    Ok(with_status(json(&result), StatusCode::CREATED))
}

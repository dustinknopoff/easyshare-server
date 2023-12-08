use std::sync::Arc;

use crate::error::AppError;
use axum::{extract, Extension, response::IntoResponse, body::Body};

use object_store::{aws::AmazonS3, path::Path, ObjectStore};

#[axum::debug_handler]
pub async fn get_object(
    extract::Path((key, file_name)): extract::Path<(String, String)>,
    Extension(s3_client): Extension<Arc<AmazonS3>>,
) -> Result<impl IntoResponse, AppError> {
    let client = s3_client.clone();

    let object = client.get(&Path::from(format!("{key}/{file_name}"))).await?;

    Ok(Body::from_stream(object.into_stream()))
}
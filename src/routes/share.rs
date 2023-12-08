use std::sync::Arc;

use crate::{error::AppError, ui::layout::layout};
use axum::{extract, response::Html, Extension};
use futures_util::StreamExt;
use maud::html;
use object_store::{aws::AmazonS3, path::Path, ObjectStore};

#[axum::debug_handler]
pub async fn list_files(
    extract::Path(id): extract::Path<String>,
    Extension(s3_client): Extension<Arc<AmazonS3>>,
) -> Result<Html<String>, AppError> {
    let client = s3_client.clone();

    let mut list_stream = client.list(Some(&Path::from(id)));


    Ok(Html(
        layout(
            "Easyshare",
            html! {
            @while let Some(meta) = list_stream.next().await.transpose().unwrap() {
            li {
                    a href={ "/obj/" (meta.location)} {
                        "Download " (meta.location)
                    }
                }
            }
                        },
        )
        .into_string(),
    ))
}
use std::sync::Arc;

use axum::{body::Bytes, response::Html, Extension};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use maud::html;
use object_store::{aws::AmazonS3, path::Path, ObjectStore, WriteMultipart};
use uuid::Uuid;

use crate::error::AppError;

#[derive(TryFromMultipart)]
pub struct UploadBody {
    #[form_data(limit = "10GiB")]
    files: Vec<FieldData<Bytes>>,
}

#[axum::debug_handler]
pub async fn upload(
    Extension(s3_client): Extension<Arc<AmazonS3>>,
    TypedMultipart(UploadBody { files }): TypedMultipart<UploadBody>,
) -> Result<Html<String>, AppError> {
    let prefix = Uuid::new_v4();

    let client = s3_client.clone();
    for file in files.into_iter() {
        let location = &Path::from(format!("{prefix}/{}", file.metadata.file_name.unwrap()));
        let upload = client.put_multipart(location).await.unwrap();
        let mut write = WriteMultipart::new(upload);
        write.write(&file.contents);
        write.finish().await?;
    }

    Ok(Html(
        html! {
            p { "Success!"}
            a href={"/share/" (prefix)} {
                "View Files"
            }
        }
        .into_string(),
    ))
}

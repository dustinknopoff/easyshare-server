use std::sync::Arc;

use axum::{Extension, body::Bytes, response::Html};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use maud::html;
use object_store::{aws::AmazonS3, path::Path, ObjectStore, PutOptions};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::error::AppError;


const GB: usize = 1024 * 1024 * 1024;


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
        if file.contents.len() > 5 * GB {
            let (_id, mut writer) = client.put_multipart(location).await?;
            writer.write_all(&file.contents).await?;
            writer.flush().await?;
            writer.shutdown().await?;
        } else {
            client
                .clone()
                .put_opts(location, file.contents, PutOptions::default())
                .await?;
        }
    }

    Ok(Html(html! {
        p { "Success!"}
        a href={"/share/" (prefix)} {
            "View Files"
        }
    }.into_string()))
}

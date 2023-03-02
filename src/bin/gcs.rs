use google_cloud_default::WithAuthExt;
use google_cloud_storage::{
    client::{Client, ClientConfig},
    http::objects::upload::{Media, UploadObjectRequest, UploadType},
};
use std::env;

#[tokio::main]
async fn main() {
    let config = ClientConfig::default().with_auth().await.unwrap();
    let client = Client::new(config);
    let bucket = env::var("BUCKET_NAME").unwrap();

    client
        .upload_object(
            &UploadObjectRequest {
                bucket: bucket,
                ..Default::default()
            },
            "hello world".as_bytes(),
            &UploadType::Simple(Media::new("file.png")),
            None,
        )
        .await
        .unwrap();
}

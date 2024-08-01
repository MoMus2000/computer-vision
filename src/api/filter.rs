use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, HttpResponse, Result, Responder, error};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

use crate::cv::sobel;

pub async fn apply_filter(mut payload: Multipart) ->  Result<NamedFile>{
    while let Some(field) = payload.next().await {
        let mut field = field.unwrap();
        let file_name = field
            .content_disposition()
            .unwrap()
            .get_filename()
            .map_or("file".to_string(), |name| name.to_string());

        println!("Got the file {}", file_name);

        let file_path = format!("./video/{}",file_name);

        let mut file = tokio::fs::File::create(&file_path).await.unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file.write_all(&data).await.unwrap();
        }

        // Handle in such a way that as the image is rendered it is sent over to the JS UI
        sobel::sobel_edge_filter(&file_path).unwrap();

        return Ok(NamedFile::open("./video/output.mp4")?);

    }
    Err(error::ErrorInternalServerError("Failed to open video file").into())
}
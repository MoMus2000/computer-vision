use actix_files::NamedFile;
use actix_web::{Result, error};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

use crate::cv::{sobel, grayscale, posterize};

pub async fn apply_filter(mut payload: Multipart) ->  Result<NamedFile>{
    let mut filter_type = String::new();
    let mut file_path = String::new();

    while let Some(field) = payload.next().await {
        let mut field = field.unwrap();

        let field_key = field
            .content_disposition()
            .unwrap()
            .get_name()
            .unwrap();

        println!("FIELD KEY : {}", field_key);

        match field_key {
            "video" => {
                let file_name = field
                    .content_disposition()
                    .ok_or_else(|| error::ErrorBadRequest("Missing content disposition"))?
                    .get_filename()
                    .map_or("file".to_string(), |name| name.to_string());
                file_path = format!("./video/{}",file_name);
                let mut file = tokio::fs::File::create(&file_path).await.unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    file.write_all(&data).await.unwrap();
                }
            },
            "filter" => {
                println!("Inside Filter");
                let mut filter_value = String::new();
                while let Some(chunk) = field.next().await {
                    let data = chunk.map_err(|e| error::ErrorInternalServerError(e))?;
                    filter_value.push_str(&String::from_utf8_lossy(&data));
                }
                filter_type = filter_value;
            }
            _ => {}
        }
    }

    match filter_type.as_str(){
        "sobel" => {
            sobel::sobel_edge_filter(&file_path).unwrap();
        },
        "grayscale"=>{
            grayscale::grayscale_filter(&file_path).unwrap();
        },
        "posterize"=>{
            posterize::posterize_filter(&file_path).unwrap();
        }
        _ => {}
    }

    return Ok(NamedFile::open("./video/output.mp4")?);
}
use actix_files::NamedFile;
use actix_web::HttpRequest;

pub async fn index(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path = format!("/Users/mmuhammad/Desktop/projects/comp_vision/src/gui/index.html");
    let res = NamedFile::open(path).unwrap();
    Ok(res)
}
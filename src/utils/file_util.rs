use actix_multipart::Multipart;
use actix_web::{web, Error};
use futures_util::stream::StreamExt;
use std::fs;
use std::path::Path;

async fn upload_file(mut payload: Multipart) -> Result<String, Error> {
  let mut file_name = String::new();

  while let Some(item) = payload.next().await {
    let mut field = item?;
    let content_type = field.content_disposition().unwrap();
    file_name = content_type.get_filename().unwrap().to_string();

    let file_path = format!("/static/orgs/logos/{}", &file_name);
    let mut f = web::block(|| fs::File::create(&file_path)).await?;

    while let Some(chunk) = field.next().await {
      let data = chunk?;
        f = web::block(|| f.write_all(&data)).await?;
      }
    }

  

    Ok(HttpResponse::Ok().body(format!("File {} uploaded successfully!", &file_name)))
}
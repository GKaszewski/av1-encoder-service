use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

use actix_multipart::Multipart;
use actix_web::{post, Error, HttpResponse, get};
use futures_util::StreamExt as _;

use crate::encode::encode_video;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("<h1>Video Converter</h1>")
}

async fn handle_file_field(mut field: actix_multipart::Field) -> Result<Option<(String, String)>, Error> {
    let content_disposition = field.content_disposition().clone();
    let field_name = content_disposition.get_name().unwrap_or_default();
    if field_name != "file" {
        return Ok(None); // Handle invalid field name
    }
    let filename = content_disposition.get_filename().unwrap_or_default();
    let filepath = format!("/tmp/{}", filename);

    println!("Filename: {}", filename);
    println!("Filepath: {}", filepath);

    if filename.is_empty() {
        println!("Missing filename");
        return Ok(None); // Handle missing filename
    }

    // Save the file temporarily
    let mut file = File::create(filepath.clone()).await?;
    println!("Created file!");
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|_| io::Error::new(io::ErrorKind::Other, "Error reading chunk"))?;
        file.write_all(&data).await?;
    }

    let filename_without_extension = filename
    .split('.')
    .next()
    .unwrap_or_default();
    let output_path = format!("/tmp/converted_{}.webm", filename_without_extension);
    Ok(Some((filepath, output_path)))
}

#[post("/convert")]
async fn convert(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut success = false;
    let mut failure = false;

    while let Some(item) = payload.next().await {
        let field = item?;
        println!("Got field: {:?}", field);
        match handle_file_field(field).await {
            Ok(None) => failure = true,
            Ok(Some((input_path, output_path))) => {
                println!("Input path: {}", input_path);
                println!("Output path: {}", output_path);
                match encode_video(&input_path, &output_path) {
                    Ok(_) => {
                        if let Err(e) = tokio::fs::remove_file(input_path).await {
                            return Ok(HttpResponse::InternalServerError().body(format!("File deletion failed: {}", e)));
                        }
                        success = true;
                    },
                    Err(_) => failure = true,
                }
            },
            Err(_) => failure = true,
        }
    }

    if success && !failure {
        return Ok(HttpResponse::Ok().body("All conversions successful"));
    } else if failure {
        return Ok(HttpResponse::InternalServerError().body("Some or all conversions failed"));
    }

    Ok(HttpResponse::BadRequest().body("Invalid request"))
}

#[post("/convert/single")]
async fn convert_single(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut success = false;
    let mut failure = false;

    while let Some(item) = payload.next().await {
        let field = item?;
        println!("Got field: {:?}", field);
        
    }

    if success && !failure {
        return Ok(HttpResponse::Ok().body("Conversion successful"));
    } else if failure {
        return Ok(HttpResponse::InternalServerError().body("Conversion failed"));
    }

    Ok(HttpResponse::BadRequest().body("Invalid request"))
}
use axum::{
    extract::{Multipart, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use futures::TryStreamExt;
use std::{fs, net::SocketAddr, path::PathBuf};
use tokio::fs::File;
use tokio_util::io::StreamReader;

const UPLOAD_DIR: &str = "uploads";
const STATIC_DIR: &str = "static";

#[tokio::main]
async fn main() {
    fs::create_dir_all(UPLOAD_DIR).expect("Failed to create upload directory");

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/upload", post(upload_file))
        .route("/{file}", get(serve_static));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn serve_index() -> impl IntoResponse {
    serve_static(Path("index.html".to_string())).await
}

async fn serve_static(Path(file): Path<String>) -> impl IntoResponse {
    let path = PathBuf::from(STATIC_DIR).join(&file);
    match tokio::fs::read(&path).await {
        Ok(contents) => {
            let content_type = match path.extension().and_then(|ext| ext.to_str()) {
                Some("html") => "text/html",
                Some("css") => "text/css",
                Some("js") => "application/javascript",
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("gif") => "image/gif",
                Some("svg") => "image/svg+xml",
                _ => "application/octet-stream",
            };
            ([("Content-Type", content_type)], contents).into_response()
        }
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}

async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("unknown");
        let file_path = PathBuf::from(UPLOAD_DIR).join(&file_name);
        let mut file = File::create(&file_path).await.unwrap();

        let mut reader = StreamReader::new(
            field.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err)),
        );

        tokio::io::copy(&mut reader, &mut file).await.unwrap();

        println!("Saved file: {}", file_path.display());
    }
    (StatusCode::OK, "File uploaded successfully").into_response()
}

use axum::{
    extract::{Multipart, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::{fs, net::SocketAddr, path::PathBuf};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

const UPLOAD_DIR: &str = "uploads";
const STATIC_DIR: &str = "static";

#[tokio::main]
async fn main() {
    fs::create_dir_all(UPLOAD_DIR).expect("Failed to create upload directory");

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/upload", post(upload_file));
    // .route("/:file", get(serve_static));

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
        Ok(contents) => (StatusCode::OK, contents).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}

async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("unknown").to_string();
        let file_path = PathBuf::from(UPLOAD_DIR).join(&file_name);
        let mut file = File::create(&file_path).await.unwrap();

        let data = field.bytes().await.unwrap();
        file.write_all(&data).await.unwrap();
        println!("Saved file: {}", file_path.display());
    }
    (StatusCode::OK, "File uploaded successfully").into_response()
}

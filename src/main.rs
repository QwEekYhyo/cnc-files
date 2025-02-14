use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use futures::TryStreamExt;
use std::{
    env, fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};
use tokio::fs::File;
use tokio_util::io::StreamReader;
use tower_http::services::ServeDir;

const UPLOAD_DIR: &str = "uploads";
const STATIC_DIR: &str = "static";

#[tokio::main]
async fn main() {
    fs::create_dir_all(UPLOAD_DIR).expect("Failed to create upload directory");

    let app = Router::new()
        .route(
            "/upload",
            post(upload_file).layer(DefaultBodyLimit::disable()),
        )
        .fallback_service(ServeDir::new(STATIC_DIR));

    let addr = env::var("ADDR")
        .ok()
        .map(|s| s.parse::<IpAddr>().expect("invalid address"));
    let port = env::var("PORT")
        .ok()
        .map(|s| s.parse::<u16>().expect("invalid port"));

    let sockaddr = SocketAddr::from((
        addr.unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
        port.unwrap_or(3000),
    ));
    println!("Listening on http://{sockaddr}");

    let listener = tokio::net::TcpListener::bind(&sockaddr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn upload_file(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    while let Some(field) = multipart.next_field().await.map_internal_err()? {
        let Some(file_name) = field.file_name() else {
            continue;
        };

        let file_path = PathBuf::from(UPLOAD_DIR).join(file_name);
        let mut file = File::create(&file_path).await.map_internal_err()?;

        let mut reader = StreamReader::new(
            field.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err)),
        );

        tokio::io::copy(&mut reader, &mut file)
            .await
            .map_internal_err()?;

        println!("Saved file: {}", file_path.display());
    }
    Ok((StatusCode::OK, "File uploaded successfully").into_response())
}

pub trait InternalErrExt<T> {
    fn map_internal_err(self) -> Result<T, (StatusCode, &'static str)>;
}

impl<T, E> InternalErrExt<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn map_internal_err(self) -> Result<T, (StatusCode, &'static str)> {
        self.inspect_err(|e| eprintln!("internal err: {e}"))
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))
    }
}

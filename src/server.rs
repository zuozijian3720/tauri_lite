use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use std::convert::Infallible;
use tokio::fs;
use tokio::net::TcpListener;

pub use super::api;

fn str_response(body: &str) -> Response<Full<Bytes>> {
    return Response::new(Full::new(Bytes::from(body.to_string())));
}

fn error_response(status_code: u16, message: String) -> Response<Full<Bytes>> {
    return Response::builder()
        .status(status_code)
        .body(Full::new(Bytes::from(message)))
        .unwrap();
}

async fn handle_static_file(
    entry_path: std::path::PathBuf,
    work_dir: std::path::PathBuf,
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let uri_path = req.uri().path();
    let file_path = if uri_path == "/" {
        entry_path
    } else {
        work_dir.join(uri_path.strip_prefix("/").unwrap())
    };

    let mime_type_result = mime_guess::from_path(&file_path).first();
    let file_result = fs::read(file_path).await;

    if file_result.is_err() {
        return Ok(error_response(404, uri_path.to_string() + " Not Found"));
    }

    let file = file_result.unwrap();
    if mime_type_result.is_some() {
        return Ok(Response::builder()
            .header("Content-Type", mime_type_result.unwrap().as_ref())
            .body(Full::new(Bytes::from(file)))
            .unwrap());
    } else {
        return Ok(Response::builder()
            .body(Full::new(Bytes::from(file)))
            .unwrap());
    }
}

async fn handle_api_call(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let body_data = req.into_body().collect().await;
    let buffer = body_data.unwrap().to_bytes().to_vec();
    let json_str = String::from_utf8(buffer).unwrap();

    return Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(api::call(json_str).await)))
        .unwrap());
}

async fn handle_request(
    entry_path: std::path::PathBuf,
    work_dir: std::path::PathBuf,
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let method = req.method();

    if method == &hyper::Method::GET {
        return handle_static_file(entry_path, work_dir, req).await;
    } else if method == &hyper::Method::POST {
        return handle_api_call(req).await;
    }

    return Ok(str_response("Not Found"));
}

#[tokio::main]
pub async fn start(
    _entry_path: std::path::PathBuf,
    _work_dir: std::path::PathBuf,
    _listener: std::net::TcpListener,
) {
    let listener = TcpListener::from_std(_listener).unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        let entry_path = _entry_path.clone();
        let work_dir = _work_dir.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    stream,
                    service_fn(move |req| {
                        return handle_request(entry_path.clone(), work_dir.clone(), req);
                    }),
                )
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
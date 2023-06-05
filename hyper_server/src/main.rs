use hyper::body::to_bytes;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("Incoming request: {} {}", req.method(), req.uri());

    if req.method() == hyper::Method::POST {
        let body_bytes = to_bytes(req.into_body()).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        println!("Request body: {}", body_str);
    }

    // Build the response body
    let response_body = "event: meta\ndata: {\"content_type\": \"text/markdown\", \"linkify\": true}\n\n\
                         event: text\ndata: {\"text\": \"I\"}\n\n\
                         event: text\ndata: {\"text\": \" am\"}\n\n\
                         event: text\ndata: {\"text\": \" the prem.\"}\n\n\
                         event: done\ndata: {}\n\n";

    // Create the response with a 200 status code and the response body
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "text/event-stream")
        .body(Body::from(response_body))
        .unwrap();

    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
